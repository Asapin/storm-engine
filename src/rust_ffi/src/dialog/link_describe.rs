use windows::Win32::Foundation::POINT;

pub struct EditConfig {
    line: i32,
    var_index: i32,
    char_index: i32,
}

pub struct DlgLinkDescribe {
    offset: POINT,
    font_id: i32,
    font_scale: f32,
    line_height: i32,
    text_color: u32,
    selected_text_color: u32,
    window_width: i32,
    selected_line: i32,
    start_index: i32,
    max_lines_per_page: i32,
    edit: Option<EditConfig>,
    current_cursor_time: f32,
    text_lines: Vec<String>,
    line_breaks: Vec<i32>,
    edit_mode: bool,
}
