use super::link_describe::DlgLinkDescribe;

pub enum SoundState {
    SoundStarting,
    SoundPlaying,
    SoundStopped
}

pub struct ScreenScale {
    x: f32,
    y: f32
}

pub struct LegacyDialog {
    link_describe: DlgLinkDescribe,
    formatted_dialog_text: Vec<String>,
    dialog_text: String,
    character_name: String,
    head_model_path: String,
    tick_sound: String,
    screen_scale: ScreenScale,
    font_scale: f32,
    line_height: i32,
    texture_lines: i32,
    main_font: i32,
    name_font: i32,
    name_color: i32,
    sub_font: i32,
    interface_texture: i32,
    back_vertex_buffer: i32,
    back_index_buffer: i32,
    fade_time: i32,
    mood: String,
    sound_state: SoundState,
    current_sound: i32,
    sound_name: String,
    dialog_needs_update: bool,
    back_needs_update: bool,
    force_emergency_close: bool,
}
