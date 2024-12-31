pub mod sdl_input;

type KeyboardKey = u32;

pub struct MousePos {
    x: i32,
    y: i32,
}

pub enum MouseKey {
    Left,
    Right,
    Middle,
    Special1,
    Special2,
}

pub enum ControllerAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
}

pub enum ControllerButton {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftBumper,
    RightBumper,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

pub struct ControllerAxisState {
    axis: ControllerAxis,
    value: i32,
}

pub enum InputEvenType {
    Unknown,
    KeyboardKeyDown(KeyboardKey),
    KeyboardKeyUp(KeyboardKey),
    KeyboardText(String),
    MouseMove(MousePos),
    MouseKeyDown(MouseKey),
    MouseKeyUp(MouseKey),
    MouseWheel,
    ControllerAxis(ControllerAxisState),
    ControllerButtonDown(ControllerButton),
    ControllerButtonUp(ControllerButton),
}

pub struct InputEvent {
    input_type: InputEvenType,
}
