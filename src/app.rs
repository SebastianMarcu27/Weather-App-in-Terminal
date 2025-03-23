use std::error;
use crate::connection::CityInfo;
use std::fs;
use std::path::Path;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// List of cities to display.
    pub cities: Vec<String>,

    /// Index of currently selected city.
    pub selected: usize,
    pub weather: Option<CityInfo>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut cities = load_cities_from_file("cities.json").unwrap_or_else(|_| {
            vec![
                "Bucharest".to_string(),
                "London".to_string(),
                "New York".to_string(),
                "Paris".to_string(),
                "Tokyo".to_string(),
            ]
        });
    
        cities.push("Exit".to_string());
    
        Self {
            running: true,
            cities,
            selected: 0,
            weather: None,
        }
    }

    /// Move selection up in the city list.
    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down in the city list.
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.cities.len() {
            self.selected += 1;
        }
    }

    /// Get currently selected city.
    pub fn selected_city(&self) -> Option<&String> {
        self.cities.get(self.selected)
    }
}

fn load_cities_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cities: Vec<String> = serde_json::from_str(&content)?;
    Ok(cities)
}
