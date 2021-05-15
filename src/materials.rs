use bevy::prelude::{ColorMaterial, Handle};

#[derive(Clone)]
pub struct Materials {
    pub hammer: Handle<ColorMaterial>,
    pub hole: Handle<ColorMaterial>,
}
