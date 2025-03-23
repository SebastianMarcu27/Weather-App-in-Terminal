use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let events_publisher = EventsPublisher::new(250);

    let mut tui = Tui::new(terminal, events_publisher);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        if let Ok(event) = tui.events.next().await {
            match event {
                Event::Key(key_event) => {
                    handle_key_events(key_event, &mut app).await?;
                }
                Event::Tick => {}
                _ => {}
            }
        }
    }

    tui.exit()?;

    Ok(())
}
