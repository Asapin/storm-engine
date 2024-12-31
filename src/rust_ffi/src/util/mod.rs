const PATH_SEPARATOR: char = '\\';
const WRONG_PATH_SEPARATOR: char = '/';

const RAND_MAX: i32 = 0x7fff;

pub fn rand(r: f32) -> f32 {
    r * (rand::random::<f32>() / (RAND_MAX as f32))
}

pub fn rand_upper(r: f32) -> f32 {
    let half = r / 2.0;
    half + rand(half)
}

pub fn rand_centered(r: f32) -> f32 {
    let half = r / 2.0;
    rand(r) - half
}
