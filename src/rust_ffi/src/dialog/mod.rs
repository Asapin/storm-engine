use link_describe::DlgLinkDescribe;
use windows::Win32::{Foundation::POINT, Graphics::Direct3D9::{D3DFVF_DIFFUSE, D3DFVF_TEX1, D3DFVF_TEXTUREFORMAT2, D3DFVF_XYZRHW}};

use crate::math::vector::Vector3;

pub mod legacy_dialog;
pub mod link_describe;

const MAX_LINES: i32 = 5;
const SCROLL_LINE_TIME: i32 = 100;
const TILED_LINE_HEIGHT: i32 = 26;
const SBL: i32 = 6;
const DEFAULT_TICK_SOUND: &str = "interface\\ok.wav";
const XI_TEX_FVF: u32 = D3DFVF_XYZRHW | D3DFVF_DIFFUSE | D3DFVF_TEX1 | D3DFVF_TEXTUREFORMAT2;
const BUTTON_STATE_UP_ENABLED: u8 = 1;
const BUTTON_STATE_DOWN_ENABLED: u8 = 2;
const BUTTON_STATE_UP_LIGHT: u8 = 4;
const BUTTON_STATE_DOWN_LIGHT: u8 = 8;

pub struct XiTexVertex {
    pos: Vector3,
    rhw: f32,
    color: u32,
    u: f32,
    v: f32,
}

pub struct DlgTextDescribe {
    offset: POINT,
    n_window_width: i32,
    n_font_id: i32,
    dw_color: u32,
    f_scale: f32,
    n_line_interval: i32,
    as_text: Vec<String>,
    n_show_quantity: i32,
    page_breaks: Vec<i32>,
    current_line: i32,
}

pub struct BackParameters {
    m_id_back_tex: i32,
    f_character_name_rect_height: f32,
    f_character_name_rect_left_width: f32,
    f_character_name_rect_center_width: f32,
    f_character_name_rect_right_width: f32,
    b_show_divider: bool,
    n_divider_height: f32,
    n_divider_offset_x: f32,
    n_divider_offset_y: i32,
}

pub struct ButtonParameters {
    m_id_texture: i32,
    f_right_offset: f32,
    f_top_offset: f32,
    f_bottom_offset: f32,
}

pub struct Dialog {
    s_talk_pers_name: String,
    m_b_dlg_changed: bool,
    m_dlg_text: DlgTextDescribe,
    link_describe: DlgLinkDescribe,
    m_back_params: BackParameters,
    m_id_v_buf_back: i32,
    m_id_i_buf_back: i32,
    m_n_v_qnt_back: i32,
    m_n_i_qnt_back: i32,
    m_button_params: ButtonParameters,
    m_id_v_buf_button: i32,
    m_id_i_buf_button: i32,
    m_n_v_qnt_button: i32,
    m_n_i_qnt_button: i32,
    m_dw_button_state: u32,
    m_n_char_name_text_font: i32,
    m_dw_char_name_text_color: u32,
    m_f_char_name_text_scale: f32,
    m_n_scr_base_width: i32,
    m_n_scr_base_height: i32,
    cur_snd: i32,
    sound_name: String,
    charDefSnd: String,
    force_emergency_close: bool,
    selected_link_name: String,
    unfade_time: i32,
    play: i32,
    start: bool,
    b_edit_mode: bool,
}
