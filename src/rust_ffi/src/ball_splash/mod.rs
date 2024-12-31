use crate::util::rand;

pub fn bsrand(r: f32) -> f32 {
    let rand_max = 0x7fff as f32;
    r * (rand::random::<f32>() / rand_max)
}

pub fn bsrand_upper(r: f32) -> f32 {
    let half = r / 2.0;
    half + rand(half)
}

pub fn bsrand_centered(r: f32) -> f32 {
    let half = r / 2.0;
    rand(r) - half
}