use std::{
    fs,
};

use macroquad::{
    experimental::{
        collections::storage,
    },
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    nodes::{
        Actor,
        ActorParams,
        GameState,
        Item,
        ItemParams,
    },
    scenario::CurrentChapter,
    GameParams,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub game_version: String,
    pub chapter: u32,
    #[serde(rename = "map")]
    pub map_id: String,
    pub player_actor_id: String,
    pub actors: Vec<ActorParams>,
    pub items: Vec<ItemParams>,
}

impl SaveGame {
    pub fn create_from_scene(game_state: &GameState) -> Self {
        let game_params = storage::get::<GameParams>();

        let player_actor_id = {
            let player = Actor::find_by_player_id(&game_state.local_player_id).unwrap();
            player.id.clone()
        };

        let mut actors = Vec::new();
        let mut items = Vec::new();

        for actor in scene::find_nodes_by_type::<Actor>() {
            for item in &actor.inventory.items {
                items.push(item.params.clone())
            }
            actors.push(actor.to_save());
        }

        for item in scene::find_nodes_by_type::<Item>() {
            items.push(item.to_params());
        }

        let current_chapter = storage::get::<CurrentChapter>();

        SaveGame {
            game_version: game_params.game_version.clone(),
            chapter: current_chapter.chapter_index as u32 + 1,
            map_id: current_chapter.current_map_id.clone(),
            player_actor_id,
            actors,
            items,
        }
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save_scene_to_file(name: &str, game_state: &GameState) {
        let game_params = storage::get::<GameParams>();
        let save_game = Self::create_from_scene(game_state);
        let path = &format!("{}/{}", game_params.saves_path, name);
        let json = serde_json::to_string_pretty(&save_game)
            .expect("Unable to serialize scene into JSON!");
        fs::write(path, &json)
            .expect(&format!("Unable to save game '{}' to disk!", path));
    }

    #[cfg(target_family = "wasm")]
    pub fn save_scene_to_file(name: &str, game_state: &GameState) {
        todo!("Implement wasm save games")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedCharacter {
    pub game_version: String,
    pub actor: ActorParams,
    pub items: Vec<ItemParams>,
}
