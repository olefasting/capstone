use macroquad::prelude::*;
use num_traits::Num;

#[derive(Debug, Copy, Clone)]
pub struct URect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl URect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        URect { x,y, w, h }
    }

    pub fn point(&self) -> UVec2 {
        uvec2(self.x, self.y)
    }

    pub fn size(&self) -> UVec2 {
        uvec2(self.w, self.h)
    }

    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> u32 {
        self.x
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> u32 {
        self.x + self.w
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> u32 {
        self.y
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> u32 {
        self.y + self.h
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn move_to(&mut self, destination: UVec2) {
        self.x = destination.x;
        self.y = destination.y;
    }

    /// Scales the `Rect` by a factor of (sx, sy),
    /// growing towards the bottom-left
    pub fn scale(&mut self, sx: u32, sy: u32) {
        self.w *= sx;
        self.h *= sy;
    }

    /// Checks whether the `Rect` contains a `Point`
    pub fn contains(&self, point: UVec2) -> bool {
        point.x >= self.left()
            && point.x < self.right()
            && point.y < self.bottom()
            && point.y >= self.top()
    }

    /// Checks whether the `Rect` overlaps another `Rect`
    pub fn overlaps(&self, other: &URect) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }

    /// Returns a new `Rect` that includes all points of these two `Rect`s.
    pub fn combine_with(self, other: URect) -> Self {
        let x = u32::min(self.x, other.x);
        let y = u32::min(self.y, other.y);
        let w = u32::max(self.right(), other.right()) - x;
        let h = u32::max(self.bottom(), other.bottom()) - y;
        URect { x, y, w, h }
    }

    /// Returns an intersection rect there is any intersection
    pub fn intersect(&self, other: URect) -> Option<Self> {
        let left = self.x.max(other.x);
        let top = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        if right < left || bottom < top {
            return None;
        }

        Some(URect {
            x: left,
            y: top,
            w: right - left,
            h: bottom - top,
        })
    }

    /// Translate rect origin be `offset` vector
    pub fn offset(self, offset: UVec2) -> Self {
        URect::new(self.x + offset.x, self.y + offset.y, self.w, self.h)
    }
}

impl From<Rect> for URect {
    fn from(rect: Rect) -> Self {
        URect {
            x: rect.x as u32,
            y: rect.y as u32,
            w: rect.w as u32,
            h: rect.h as u32,
        }
    }
}

impl From<(UVec2, UVec2)> for URect {
    fn from((position, size): (UVec2, UVec2)) -> Self {
        URect {
            x: position.x,
            y: position.y,
            w: size.x,
            h: size.y,
        }
    }
}

impl From<URect> for Rect {
    fn from(rect: URect) -> Self {
        Rect {
            x: rect.x as f32,
            y: rect.y as f32,
            w: rect.w as f32,
            h: rect.h as f32,
        }
    }
}
