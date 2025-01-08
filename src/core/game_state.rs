#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Start,
    Playing,
    Paused,
    GameOver,
} 