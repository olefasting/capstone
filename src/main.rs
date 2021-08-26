#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]
#![feature(async_closure)]

use macroquad::{
    experimental::{
        collections::storage,
        scene,
    },
    prelude::*,
};

use config::Config;
use gui::skins::GuiSkins;
use map::{
    Map,
    MapCollisionKind,
    TiledMapDeclaration,
};
use nodes::{
    Actor,
    Camera,
    ContinuousBeams,
    DrawBuffer,
    GameState,
    Hud,
    Item,
    PostProcessing,
    Projectiles,
};
use nodes::item::Credits;
use resources::Resources;
pub use uid::generate_id;
use crate::scenario::{Scenario, ScenarioParams};
use crate::modules::load_modules;

pub mod resources;
pub mod ability;
pub mod map;
pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;
pub mod math;
pub mod gui;
pub mod json;
pub mod helpers;
pub mod missions;
pub mod config;
pub mod uid;
pub mod modules;
pub mod dialogue;
pub mod scenario;

fn window_conf() -> Conf {
    let config = Config::load();

    Conf {
        window_title: "Capstone".to_owned(),
        high_dpi: true,
        window_width: config.resolution.x as i32,
        window_height: config.resolution.y as i32,
        fullscreen: config.fullscreen,
        ..Default::default()
    }
}

pub fn load_map(chapter: u32, map_id: &str) {
    let player_id = generate_id();

    let scenario = storage::get::<Scenario>();
    let chapter_data = scenario.chapters.get(chapter as usize)
        .expect(&format!("Unable to load chapter '{}'!", chapter));
    let map_data = chapter_data.maps.iter().find(|map| map.id == map_id)
        .expect(&format!("Unable to load map '{}' of chapter '{}'!", map_id, chapter_data.title));

    scene::clear();

    GameState::add_node(map_data.map.clone(), &player_id);
    Camera::add_node();
    DrawBuffer::<Item>::add_node();
    DrawBuffer::<Credits>::add_node();
    Projectiles::add_node();
    ContinuousBeams::add_node();
    DrawBuffer::<Actor>::add_node();
    PostProcessing::add_node();
    Hud::add_node();
}

const TILED_MAPS_FILE_PATH: &'static str = "assets/tiled_maps.json";

#[macroquad::main(window_conf)]
async fn main() {
    {
        let config = storage::get::<Config>();
        storage::store(GuiSkins::new(config.gui_scale));

        let mut resources = Resources::new().await.unwrap();
        let mut scenario_params = Scenario::load_params().await.unwrap();
        load_modules(&mut resources, &mut scenario_params).await;

        storage::store(resources);

        let bytes = load_file(TILED_MAPS_FILE_PATH).await
            .expect(&format!("Unable to find tiled maps file '{}'!", TILED_MAPS_FILE_PATH));
        let tiled_maps: Vec<TiledMapDeclaration> = serde_json::from_slice(&bytes)
            .expect(&format!("Unable to parse tiled maps file '{}'!", TILED_MAPS_FILE_PATH));
        for decl in tiled_maps {
            Map::load_tiled(decl.clone()).await
                .expect(&format!("Unable to convert tiled map '{}'!", decl.path));
        }

        let scenario = Scenario::new(scenario_params).await.unwrap();
        storage::store(scenario);
    }

    let (chapter_i, map_id) = gui::draw_chapter_select().await;

    load_map(chapter_i, &map_id);

    loop {
        {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.should_quit {
                break;
            }
        }

        gui::draw_gui();

        next_frame().await;
    }

    scene::clear();

    let config = storage::get::<Config>();
    config.save();
}
