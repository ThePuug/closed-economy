pub mod hx;
pub mod keybits;

use bevy::prelude::*;
use bevy_easings::*;
use serde::{Deserialize, Serialize};

use crate::common::components::hx::*;
pub trait IntoScreen {
    fn into_screen(self) -> Vec3;
}

pub trait Calculate<T> {
    fn calculate(self) -> T;
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct DecoratorDescriptor {
    pub index: usize,
    pub is_solid: bool,
}

#[derive(Clone, Component, Copy, Debug, Deserialize, Serialize)]
pub enum EntityType {
    Actor,
    Decorator(DecoratorDescriptor),
}

#[derive(Clone, Component, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Offset(pub Vec3);

impl Lerp for Offset {
    type Scalar = f32;
    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        Offset(self.0.lerp(other.0, *scalar))
    }
}

impl Calculate<Vec3> for (Hx, Offset) {
    fn calculate(self) -> Vec3 {
        // z-offset is game coords
        let v: Vec3 = Vec3{ z: self.0.z as f32 + self.1.0.z, ..self.0.into()};
        // normalize to orthographic range
        let z = ((v.z - self.0.r as f32 * 100.) / 2_i32.pow(16) as f32) * 1000.;
        // xy-offset are screen coords
        v + Vec3 { x: 0., y: v.z * TILE_RISE, z } + self.1.0.xy().extend(0.)
    }
}

impl Into<Offset> for Heading {
    fn into(self) -> Offset {
        Offset(Vec2::default().lerp(Vec3::from(self.0).xy(), 0.25).extend(self.0.z as f32))
    }
}

#[derive(Clone, Component, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Heading(pub Hx);

#[derive(Clone, Component, Copy, Debug, Default, Deserialize, Serialize)]
pub struct AirTime(pub i16);

#[derive(Clone, Component, Copy, Default)] 
pub struct Actor;

