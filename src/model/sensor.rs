pub struct GyroInfo {
    x: f32,
    y: f32,
    z: f32,
}

impl GyroInfo {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

pub struct AccInfo {
    x: f32,
    y: f32,
    z: f32,
}

impl AccInfo {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}
