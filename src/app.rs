use crate::event::Event;
use crate::store::Action;
use crate::view::{HomeProps, View};
use crate::{event::EventHandler, store::AppStore, tui::Tui};

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;

use ratatui::{backend::CrosstermBackend, Terminal};

pub struct App {
    pub store: AppStore,
    pub tui: Tui,
    pub events: EventHandler,
}

impl App {
    pub fn try_new() -> Result<Self> {
        let backend = CrosstermBackend::new(std::io::stderr());
        let terminal = Terminal::new(backend)?;
        let events = EventHandler::new(50);

        Ok(Self {
            store: AppStore::default(),
            tui: Tui::new(terminal),
            events,
        })
    }
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        // enter the terminal ui
        self.tui.enter()?;

        enum State {
            Running,
            Off,
        }

        let mut state = State::Running;

        let increment_amount:isize = 10;

        // Start the main loop
        while let State::Running = state {
            self.tui.terminal.draw(|f| {
                let view =  View::Home(HomeProps {
                    counter: self.store.state.get_counter(),
                    screen_size: f.size(),
                    increment_amount
                })
                .build();

                for (component, area) in view {
                    f.render_widget(
                        component, area
                    );
                }
            })?;

            match self.events.next()? {
                Event::Tick => self.store.reduce(),
                Event::Key(key_event) => {
                    use KeyCode::Char;
                    match key_event.code {
                        Char('q') => state = State::Off,
                        Char(' ') => self.store.dispatch(Action::Increment(increment_amount)),
                        KeyCode::Left => self.store.dispatch(Action::Increment(-1)),
                        KeyCode::Right => self.store.dispatch(Action::Increment(1)),
                        _ => {}
                    }
                }
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            };
        }

        // exit the terminal ui
        self.tui.exit()?;

        Ok(())
    }
}
