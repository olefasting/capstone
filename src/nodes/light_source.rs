use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
        },
    },
    color,
    prelude::*,
};

pub struct LightSource {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub intensity: f32,
}

impl LightSource {
    pub const DEFAULT_SIZE: Vec2 = Vec2::ZERO;
    pub const DEFAULT_COLOR: Color = color::WHITE;
    pub const DEFAULT_INTENSITY: f32 = 0.1;

    pub fn new(position: Vec2, size: Vec2, color: Color, intensity: f32) -> Self {
        LightSource {
            position,
            size,
            color,
            intensity,
        }
    }

    pub fn add_node(position: Vec2, size: Vec2, color: Color, intensity: f32) -> Handle<Self> {
        scene::add_node(Self::new(position, size, color, intensity))
    }
}

impl Node for LightSource {}
