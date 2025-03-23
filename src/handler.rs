use crate::app::{App, AppResult};
use crate::connection::get_data;
use crossterm::event::{KeyCode, KeyEvent};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') => {
            app.running = false;
        }
        KeyCode::Up => {
            app.select_previous();
        }
        KeyCode::Down => {
            app.select_next();
        }
        KeyCode::Enter => {
            if let Some(city) = app.selected_city() {
                if city == "Exit" {
                    app.running = false;
                } else {
                    match get_data(city.to_string()).await {
                        Ok(info) => app.weather = Some(info),
                        Err(err) => {
                            eprintln!("Error fetching weather data: {err}");
                            app.weather = None;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
