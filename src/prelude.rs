pub use std::{
    collections::HashMap,
    iter::FromIterator,
};

pub use macroquad::{
    self,
    experimental::{
        collections::storage,
        scene::{
            Node,
            RefMut,
            Handle,
        },
        animation::{
            AnimatedSprite,
            Animation,
            AnimationFrame,
        }
    },
    audio::{
        Sound,
        load_sound,
        play_sound,
        play_sound_once,
    },
    color,
    prelude::*,
};

pub use serde::{
    Serialize,
    Deserialize,
};

pub use crate::{
    TOOLKIT_VERSION,
    versions::{
        check_version_requirement,
        is_version_up_to_or,
    },
    config::Config,
    uid::generate_id,
    resources::Resources,
    noise_level::NoiseLevel,
    scenario::{
        Scenario,
        ScenarioParams,
        Chapter,
        ChapterParams,
        CurrentChapter,
        SceneTransition,
        SceneTransitionParams,
    },
    save_games::{
        SaveGame,
        ExportedCharacter,
    },
    physics::{
        Collider,
        PhysicsBody,
        raycast,
    },
    inventory::{
        Inventory,
        InventoryParams,
        InventoryEntry,
        EquippedItems,
        EquipmentSlot,
    },
    math::{
        Circle,
        URect,
    },
    nodes::{
        actor::{
            Actor,
            ActorParams,
            ActorBehavior,
            ActorBehaviorParams,
            ActorStats,
            ActorController,
            ActorControllerKind,
            apply_actor_behavior,
        },
        LightSource,
        Camera,
        ContinuousBeams,
        draw_buffer::{
            DrawBuffer,
            BufferedDraw,
            Bounds,
        },
        GameState,
        Hud,
        item::{
            Item,
            ItemKind,
            ItemParams,
        },
        PostProcessing,
        projectiles::{
            Projectiles,
            ProjectileKind,
        },
        Credits,
    },
    game::{
        run_game,
        GameParams,
    },
    gui::{
        self,
        skins::GuiSkins,
    },
    map::{
        Map,
        MapLayer,
        MapLayerKind,
        MapCollisionKind,
        MapTileset,
        MapObject,
        MapTile,
    },
    missions::{
        Mission,
        MissionParams,
        MissionReward,
        MissionObjective,
    },
    dialogue::{
        Dialogue,
        DialogueAction,
        DialogueRequirement,
    },
    render::{
        draw_progress_bar,
        draw_aligned_text,
        try_color_from_hex_string,
        color_from_hex_string,
        Sprite,
        SpriteAnimationParams,
        SpriteAnimationPlayer,
        Viewport,
        HorizontalAlignment,
        VerticalAlignment,
    },
    ability::{
        Ability,
        AbilityParams,
        AbilityDelivery,
        DamageType,
        Effect,
    },
    input::{
        self,
        map_gamepad,
        get_gamepad,
        get_mapped_gamepad,
        get_gamepad_id,
        get_events,
        get_player_id,
        get_mouse_position,
        get_mouse_in_world_space,
        update_input,
        apply_input,
    },
    json,
};
