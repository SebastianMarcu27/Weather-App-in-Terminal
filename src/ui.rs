use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // 1. Split the layout in vertical (2 zones: left = city list, right = weather info)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(frame.size());

    // 2. Build list of cities as ListItem
    let cities: Vec<ListItem> = app
        .cities
        .iter()
        .map(|city| ListItem::new(Line::from(vec![Span::raw(city)])))
        .collect();

    let list = List::new(cities)
        .block(Block::default().title("Cities").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    // 3. Keep list state so selected item is shown
    let mut state = ListState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(list, chunks[0], &mut state);

    // 4. Create weather info panel
    let weather_info = if let Some(info) = &app.weather {
        format!(
            "City: {}\nTemp: {:.1}°C\nHumidity: {}%\nWind: {:.1} m/s\nTime: {}",
            info.city,
            info.temperature,
            info.humidity,
            info.wind_speed,
            info.time.format("%Y-%m-%d %H:%M:%S")
        )
    } else {
        "Press Enter to fetch weather data.".to_string()
    };

    let paragraph = Paragraph::new(weather_info)
        .block(Block::default().title("Weather Info").borders(Borders::ALL));

    frame.render_widget(paragraph, chunks[1]);
}
