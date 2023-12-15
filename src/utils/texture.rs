use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub(crate) struct Size {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl std::ops::Mul<f32> for Size {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Asset {
    pub(crate) size: Size,
    pub(crate) scale: f32,
    pub(crate) texture: String,
}

impl Asset {
    pub(crate) fn get_size(&self) -> Size {
        self.size * self.scale
    }
}
