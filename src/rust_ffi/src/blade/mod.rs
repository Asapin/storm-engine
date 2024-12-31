use windows::Win32::Graphics::Direct3D9::{D3DFVF_DIFFUSE, D3DFVF_XYZ};

const ITEMS_INFO_QUANTITY: usize = 10;
const FVF: u32 = D3DFVF_XYZ | D3DFVF_DIFFUSE;
const WAY_LENGTH: usize = 64;

pub struct Vertex {
    diffuse: u32
}

pub struct BladeInfo {
    color: [i32; 2],
    def_life_time: f32,
    life_time: f32,
    time: f32,
    vrt: [Vertex; WAY_LENGTH * 2],
    vrt_time: [f32; WAY_LENGTH],
    id: String,
    locator_name: String,
    locator_name_active: String,
    active: bool,
}

pub struct TieItemInfo {
    n_item_index: i32,
    locator_name: String,
}

pub struct Blade {
    blend_value: u32,
    blades: [BladeInfo; 2],
    gun_locator: String,
    gun_locator_active: String,
    gun_active: bool,
    items: [TieItemInfo; ITEMS_INFO_QUANTITY],
}
