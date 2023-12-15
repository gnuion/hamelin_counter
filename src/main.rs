use color_eyre::eyre::Result;

mod app;
mod event;
mod store;
mod tui;
mod view;

fn main() -> Result<()> {
    let mut app = app::App::try_new()?;
    app.run()?;

    Ok(())
}
