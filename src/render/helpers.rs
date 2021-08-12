use macroquad::{
    color,
    prelude::*,
};

pub enum HorizontalAlignment {
    Left,
    Right,
    Center,
}

pub fn draw_progress_bar(
    value: f32,
    max_value: f32,
    position: Vec2,
    length:f32,
    height: f32,
    color: Color,
    bg_color: Color,
    border: f32,
    alignment: HorizontalAlignment,
) {
    assert!(border * 2.0 < height && border * 2.0 < length, "Progress bar length and height must be greater than border * 2");
    {
        let coords = match alignment {
            HorizontalAlignment::Left => (
                position.x,
                position.y,
                position.x + length,
                position.y,
            ),
            HorizontalAlignment::Center => (
                position.x - length / 2.0,
                position.y,
                position.x + length / 2.0,
                position.y,
            ),
            HorizontalAlignment::Right => (
                position.x + length,
                position.y,
                position.x + length * 2.0,
                position.y,
            ),
        };
        draw_line(
            coords.0,
            coords.1,
            coords.2,
            coords.3,
            height,
            bg_color,
        );
    }
    {
        let coords = match alignment {
            HorizontalAlignment::Left => (
                position.x + border,
                position.y,
                position.x + (value / max_value) * length - border,
                position.y,
            ),
            HorizontalAlignment::Center => (
                position.x + border - length / 2.0,
                position.y,
                position.x + (value / max_value) * (length - border) - length / 2.0,
                position.y,
            ),
            HorizontalAlignment::Right => (
                position.x + border + length,
                position.y,
                position.x + (value / max_value) * (length - border) + length * 2.0,
                position.y,
            ),
        };
        draw_line(
            coords.0,
            coords.1,
            coords.2,
            coords.3,
            height - border * 2.0,
            color,
        );
    }
}