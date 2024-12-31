pub enum Event {
    Unknown,
    FocusGained, // Window gained focus
    FocusLost, // Window lost focus
    Closed // Window was closed by user
}

pub struct WindowSize {
    width: i32,
    height: i32
}
