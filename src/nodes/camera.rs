use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
    },
    color,
    prelude::*,
};

use crate::{set_global, render::{
    get_aspect_ratio,
    to_world_space,
    to_screen_space,
    Viewport,
}, nodes::{
    Actor,
}, get_mouse_position, draw_aligned_text, get_global};
use std::ops::Sub;
use crate::render::HorizontalAlignment;

pub struct Camera {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: f32,
    zoom_speed: f32,
    pan_speed: f32,
    rotation_speed: f32,
}

impl Camera {
    const FOLLOW_THRESHOLD: f32 = 0.35;

    const DEFAULT_PAN_SPEED: f32 = 50.0;
    const DEFAULT_ROTATION_SPEED: f32 = 75.0;
    const DEFAULT_ZOOM_SPEED: f32 = 0.75;
    const DEFAULT_SCALE: f32 = 3.0;

    const ZOOM_MIN: f32 = 1.0;
    const ZOOM_MAX: f32 = 6.0;

    pub fn new(position: Vec2) -> Self {
        Camera {
            position,
            rotation: 0.0,
            scale: Self::DEFAULT_SCALE,
            zoom_speed: Self::DEFAULT_ZOOM_SPEED,
            pan_speed: Self::DEFAULT_PAN_SPEED,
            rotation_speed: Self::DEFAULT_ROTATION_SPEED,
        }
    }

    pub fn add_node(position: Vec2) -> Handle<Self> {
        scene::add_node(Camera::new(position))
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        get_aspect_ratio()
    }

    pub fn get_viewport(&self) -> Viewport {
        let width = screen_width() / self.scale;
        let height = screen_height() / self.scale;
        Viewport {
            x: self.position.x - (width / 2.0),
            y: self.position.y - (height / 2.0),
            width,
            height,
            scale: self.scale,
        }
    }

    pub fn to_screen_space(&self, coords: Vec2) -> Vec2 {
        to_screen_space(coords, self.get_viewport().get_view_rect().point(), self.scale)
    }

    pub fn to_world_space(&self, coords: Vec2) -> Vec2 {
        to_world_space(coords, self.get_viewport().get_view_rect().point(), self.scale)
    }

    pub fn get_mouse_world_coords(&self) -> Vec2 {
        self.to_world_space(get_mouse_position())
    }

    pub fn pan(&mut self, direction: Vec2) {
        self.position.x += direction.x * (self.pan_speed);
        self.position.y -= direction.y * (self.pan_speed);
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.rotation += rotation.clamp(-self.rotation_speed, self.rotation_speed);
    }

    pub fn rotate_cw(&mut self) {
        self.rotation += self.rotation_speed;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation -= self.rotation_speed;
    }

    pub fn zoom(&mut self, zoom: f32) {
        let zoom = self.scale + (zoom * self.zoom_speed).clamp(-self.zoom_speed, self.zoom_speed);
        self.scale = zoom.clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
    }

    pub fn zoom_in(&mut self) {
        let zoom = self.scale - self.zoom_speed;
        self.scale = zoom.clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
    }

    pub fn zoom_out(&mut self) {
        let zoom = self.scale + self.zoom_speed;
        self.scale = zoom.clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
    }
}

impl Node for Camera {
    fn ready(node: RefMut<Self>) {
        set_global(node.get_viewport());
    }

    fn update(node: RefMut<Self>) {
        set_global(node.get_viewport());
    }

    fn fixed_update(mut node: RefMut<Self>) {
        let actor = Actor::find_local_player().unwrap();
        let viewport = node.get_viewport();
        let mod_size = vec2(viewport.width * Self::FOLLOW_THRESHOLD, viewport.height * Self::FOLLOW_THRESHOLD);
        let bounds = Rect::new(
            viewport.x + (viewport.width - mod_size.x) / 2.0,
            viewport.y + (viewport.height - mod_size.y) / 2.0,
            mod_size.x,
            mod_size.y,
        );
        if !bounds.contains(actor.body.position) {
            let direction = actor.body.position.sub(node.position).normalize_or_zero();
            node.position += direction * actor.stats.move_speed;
        }
    }

    fn draw(node: RefMut<Self>) {
        push_camera_state();
        set_default_camera();
        draw_aligned_text(
            &format!("camera: {}", node.position.to_string()) ,
            screen_width() - 50.0,
            100.0,
            HorizontalAlignment::Right,
            Default::default(),
        );
        pop_camera_state();

        scene::set_camera_1(Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(node.position.x, node.position.y),
            zoom: vec2(node.scale / screen_width(), -node.scale / screen_height()) * 2.0,
            rotation: node.rotation,
            ..Camera2D::default()
        });
    }
}
