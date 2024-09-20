pub mod message;
pub mod keybits;

use std::ops::Add;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::hx::*;

pub const KEYBIT_UP: u8 = 1 << 0;
pub const KEYBIT_DOWN: u8 = 1 << 1; 
pub const KEYBIT_LEFT: u8 = 1 << 2; 
pub const KEYBIT_RIGHT: u8 = 1 << 3; 
pub const KEYBIT_ZOOM_OUT: u8 = 1 << 4;
pub const KEYBIT_ZOOM_IN: u8 = 1 << 5;

pub trait IntoScreen {
    fn into_screen(self) -> Vec3;
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
pub struct Pos {
    pub hx: Hx,
    pub offset: Vec3,
}

impl IntoScreen for Pos {
    fn into_screen(self) -> Vec3 {
        let v: Vec3 = self.hx.into();
        Vec3 { z: (self.hx.z - self.hx.r) as f32, ..v } + self.offset
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos { hx: self.hx + rhs.hx, offset: self.offset + rhs.offset }
    }
}

#[derive(Clone, Component, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Heading(pub Hx);

#[derive(Clone, Component, Copy, Default)] 
pub struct Actor;