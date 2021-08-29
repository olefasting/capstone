use std::ops::Sub;

use macroquad::prelude::*;

use crate::{
    nodes::GameState,
    map::MapCollisionKind,
    nodes::Actor,
};

use super::{
    ACTOR_TO_ACTOR_COLLISIONS,
    COLLISION_RESOLUTION,
    Collider,
};

pub fn raycast(origin: Vec2, end: Vec2, ignore_barriers: bool) -> Option<Vec2> {
    if origin.distance(end) > COLLISION_RESOLUTION {
        let direction = end.sub(origin).normalize_or_zero();
        let game_state = scene::find_node_by_type::<GameState>().unwrap();
        let collider = Collider::circle(0.0, 0.0, 1.0);
        let change = direction * COLLISION_RESOLUTION;
        let mut current = origin;
        while current.distance(end) > COLLISION_RESOLUTION {
            let collider = collider.offset(current);
            for (_, kind) in game_state.map.get_collisions(collider) {
                if ignore_barriers == false || kind == MapCollisionKind::Solid {
                    return Some(current);
                }
            }
            if ACTOR_TO_ACTOR_COLLISIONS {
                for actor in scene::find_nodes_by_type::<Actor>() {
                    if let Some(other_collider) = actor.body.get_offset_collider() {
                        if other_collider.contains(current) {
                            return Some(current);
                        }
                    }
                }
            }
            current += change;
        }
    }
    None
}
