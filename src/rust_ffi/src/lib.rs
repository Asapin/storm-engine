use diagnostics::logs;

pub mod animals;
pub mod animation;
pub mod ball_splash;
pub mod battle_interface;
pub mod blade;
pub mod blot;
pub mod collide;
pub mod config;
pub mod core;
pub mod diagnostics;
pub mod dialog;
pub mod editor;
pub mod ffi_util;
pub mod geometry;
pub mod input;
pub mod island;
pub mod lighter;
pub mod location;
pub mod locator;
pub mod mast;
pub mod math;
pub mod model;
pub mod particles;
pub mod pcs_controls;
pub mod renderer;
pub mod rigging;
pub mod sailors;
pub mod script_library;
pub mod sea;
pub mod sea_ai;
pub mod sea_cameras;
pub mod sea_creatures;
pub mod sea_foam;
pub mod sea_operator;
pub mod shadow;
pub mod shared_headers;
pub mod ship;
pub mod sink_effect;
pub mod sound;
pub mod sound_service;
pub mod teleport;
pub mod tornado;
pub mod touch;
pub mod util;
pub mod water_rings;
pub mod weather;
pub mod window;
pub mod worldmap;
pub mod xinterface;

#[no_mangle]
pub extern "C" fn init() {
    logs::init();
}
