use num_enum::IntoPrimitive;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Transformation {
    Normal = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
    Flip = 4,
    FlipRotate90 = 5,
    FlipRotate180 = 6,
    FlipRotate270 = 7,
}

impl Transformation {
    pub fn code(&self) -> u8 {
        (*self).into()
    }

    pub fn from_code(code: u8) -> Option<Self> {
        Transformation::try_from(code).ok()
    }
}

impl Serialize for Transformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.code())
    }
}

impl<'de> Deserialize<'de> for Transformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u8::deserialize(deserializer)?;
        Ok(Transformation::from_code(code).unwrap_or(Transformation::Normal))
    }
}
