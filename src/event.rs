use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

pub enum Event {
    /// Terminal tick
    Tick,
    /// Key press
    Key(KeyEvent),
    /// Mouse click/scroll
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
}

use std::{
    sync::mpsc::{self, RecvError},
    thread,
    time::{Duration, Instant},
};
pub struct EventHandler {
    /// Event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Construct a new instance of [`EventHandler`]
    pub fn new(tick_delay: u64) -> Self {
        // Define tick rate. To get the fps, you divide 1 by tick rate.
        let tick_rate = Duration::from_millis(tick_delay);
        // Define sender and receiver for handling events
        let (sender, receiver) = mpsc::channel();
        // Spawn a new process to handle events in the background.
        let handler = {
            // We clone the sender because we are going to move it to a new thread
            let sender = sender.clone();
            thread::spawn(move || {
                // We set last tick to current moment to start our logic
                let mut last_tick = Instant::now();
                loop {
                    // Check if we are within the tickrate and set the time left as the timeout.
                    // If too much time has passed, and therefor the timeout is negative, we set
                    // the timeout to be the tick_rate which is positive.
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    // Listen for an event for the duration of the timeout.
                    if event::poll(timeout).expect("unable to pool for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    // Send the event via the mpsc sender
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(())
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }
                    // Check if the duration reached tick_rate and if so send a tick event.
                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    /// 
    /// This function will always block the current thread if
    /// there is no data available '
    pub fn next(&self) -> Result<Event, RecvError> {
        Ok(self.receiver.recv()?)
    }
}
