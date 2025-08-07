// Configuration file for Minimalist Game Prototype Tracker

// Game settings
pub struct GameSettings {
    pub title: String,
    pub version: String,
    pub goal: String,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            title: "Minimalist Game Prototype".to_string(),
            version: "v0.1".to_string(),
            goal: "Reach 10 points in 5 minutes".to_string(),
        }
    }
}

// Tracker settings
pub struct TrackerSettings {
    pub max_entries: usize,
    pub display_limit: usize,
}

impl Default for TrackerSettings {
    fn default() -> Self {
        Self {
            max_entries: 100,
            display_limit: 10,
        }
    }
}

// Game state
pub struct GameState {
    pub score: i32,
    pub time_left: i32,
    pub entries: Vec<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            time_left: 300, // 5 minutes
            entries: Vec::new(),
        }
    }
}

// Configuration
pub struct Config {
    pub game_settings: GameSettings,
    pub tracker_settings: TrackerSettings,
    pub game_state: GameState,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_settings: GameSettings::default(),
            tracker_settings: TrackerSettings::default(),
            game_state: GameState::default(),
        }
    }
}