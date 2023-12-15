use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

pub struct HomeProps {
    pub counter: isize,
    pub screen_size: Rect,
    pub increment_amount: isize,
}

pub enum View {
    Home(HomeProps),
}

impl View {
    pub fn build(&self) -> Vec<(Paragraph<'static>, Rect)> {
        match self {
            View::Home(props) => {
                let mut widgets = Vec::new();

                // Counter
                let area = Layout::new()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50 - 300 / props.screen_size.height),
                        Constraint::Length(2),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ])
                    .split(props.screen_size);

                widgets.push((
                    Paragraph::new(format!("Counter: {}", props.counter))
                        .alignment(Alignment::Center),
                    area[1],
                ));
                widgets.push((
                    Paragraph::new(format!(
                        "Hit space to increment counter by {}",
                        props.increment_amount
                    ))
                    .alignment(Alignment::Center),
                    area[2],
                ));
                widgets.push((
                    Paragraph::new("Hit Left Arrow to decrement").alignment(Alignment::Center),
                    area[3],
                ));
                widgets.push((
                    Paragraph::new(format!(
                        "Hit Right Arrow to increment",
                    ))
                    .alignment(Alignment::Center),
                    area[4],
                ));
                widgets.push((
                    Paragraph::new(format!("Hit q to quit"))
                        .alignment(Alignment::Center),
                    area[5],
                ));

                // Hint

                widgets
            }
        }
    }
}
