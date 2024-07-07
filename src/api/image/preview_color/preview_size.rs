use {
    serde_repr::{Deserialize_repr, Serialize_repr},
    utoipa::ToSchema,
};

#[derive(Debug, Deserialize_repr, Serialize_repr, ToSchema)]
#[repr(u8)]
#[schema(default = 1)]
pub enum PreviewSize {
    Small,
    Medium,
    Large,
}

impl From<u8> for PreviewSize {
    fn from(value: u8) -> Self {
        use PreviewSize::*;

        match value {
            1 => Small,
            2 => Medium,
            3 => Large,
            _ => Small,
        }
    }
}

impl From<PreviewSize> for (u32, u32) {
    fn from(val: PreviewSize) -> Self {
        use PreviewSize::*;

        match val {
            Small => (128, 128),
            Medium => (256, 256),
            Large => (512, 512),
        }
    }
}
