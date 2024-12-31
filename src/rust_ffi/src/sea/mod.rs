use crate::math::vector::Vector3;

pub struct SeaVertex {
    pub v_pos: Vector3,
    pub v_normal: Vector3,
    pub tu: f32,
    pub tv: f32
}

pub struct SeaBlock {
    pub ix1: i32,
    pub ix2: i32,
    pub iy1: i32,
    pub iy2: i32,
    pub isize0: i32,

    pub iTX: i32,
    pub iTY: i32,
    pub iSize: i32,
    pub iLOD: i32,
    pub iIStart: i32,
    pub iIFirst: i32,
    pub iILast: i32,

    pub bInProgress: bool,
    pub bDone: bool
}

impl SeaBlock {
    pub fn QSort(b1: &Self, b2: &Self) -> bool {
        return b1.iLOD > b2.iLOD;
    }
}

impl PartialEq for SeaBlock {
    fn eq(&self, other: &Self) -> bool {
        self.iLOD == other.iLOD
    }
}

impl PartialOrd for SeaBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iLOD.partial_cmp(&other.iLOD)
    }
}

impl Eq for SeaBlock {
    
}

impl Ord for SeaBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iLOD.cmp(&other.iLOD)
    }
}

pub struct SeaTrash {
    pub vPos: Vector3,
    pub vSpeed: Vector3,
    pub dwSubTexture: u32,
    pub fTime: f32,
    pub fSize: f32
}

pub struct SeaLight {
    pub vPos: Vector3,
    pub vSpeed: Vector3,
    pub dwSubTexture: u32,
    pub fTime: f32,
}