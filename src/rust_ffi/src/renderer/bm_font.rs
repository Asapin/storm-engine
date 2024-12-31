use std::path::PathBuf;

pub struct BmCharacter {
    id: char,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    x_offset: i16,
    y_offset: i16,
    x_advance: i16,
    page: u8,
    channel: u8
}

pub struct BmKerning {
    first: char,
    second: char,
    amount: i16
}

pub struct BmTexture {
    texture_path: PathBuf,
    texture_handle: i32
}

pub struct UpdateVertexBufferResult {
    characters: char,
    x_offset: f32
}

pub struct BmFont {
    characters: Vec<BmCharacter>,
    kernings: Vec<BmKerning>,
    textures: Vec<BmTexture>,
    scale: f64,
    line_height_scaling: f64,
    line_height: usize,
    base: usize,
    texture_width: usize,
    texture_height: usize,
    vertex_buffer: i32,
    gradient_texture: i32,
}
