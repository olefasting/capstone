use macroquad::{
    experimental::{
        scene::RefMut,
        collections::storage,
    },
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    nodes::item::{
        ItemKind,
        ItemParams,
        Item,
        Credits,
    },
    ability::Ability,
    generate_id,
    resources::Resources,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorInventoryParams {
    pub items: Vec<String>,
    #[serde(default)]
    pub credits: u32,
}

impl Default for ActorInventoryParams {
    fn default() -> Self {
        ActorInventoryParams {
            items: Vec::new(),
            credits: 0,
        }
    }
}

#[derive(Clone)]
pub struct ActorInventoryEntry {
    pub id: String,
    pub params: ItemParams,
}

impl ActorInventoryEntry {
    pub fn new(id: Option<String>, params: ItemParams) -> Self {
        ActorInventoryEntry {
            id: id.unwrap_or(generate_id()),
            params,
        }
    }
}

impl ActorInventoryEntry {
    pub fn to_actor_ability(&self) -> Ability {
        Ability::new(self.params.ability.clone())
    }
}

#[derive(Clone)]
pub struct ActorInventory {
    pub items: Vec<ActorInventoryEntry>,
    pub credits: u32,
}

impl ActorInventory {
    const DROP_ALL_POSITION_VARIANCE: f32 = 15.0;

    pub fn new() -> Self {
        ActorInventory {
            items: Vec::new(),
            credits: 0,
        }
    }

    pub fn get_all_of_kind(&self, kinds: &[ItemKind]) -> Vec<ActorInventoryEntry> {
        self.items.clone().into_iter().filter(|item| {
            if kinds.contains(&item.params.kind) {
                return true;
            }
            false
        }).collect()
    }

    pub fn pick_up(&mut self, item: RefMut<Item>) {
        self.items.push(ActorInventoryEntry::new(Some(item.id.clone()),ItemParams::from(&*item)));
        item.delete();
    }

    pub fn add_item(&mut self, item_params: ItemParams) {
        self.items.push(ActorInventoryEntry::new(None, item_params));
    }

    pub fn add_credits(&mut self, amount: u32) {
        self.credits += amount;
    }

    pub fn drop(&mut self, item_id: &str, position: Vec2) -> bool {
        let items: Vec<ActorInventoryEntry> = self.items
            .drain_filter(|entry| {
                if entry.id == item_id && entry.params.is_quest_item == false {
                    let params = ItemParams {
                        position: Some(Self::randomize_drop_position(position)),
                        ..entry.params.clone()
                    };
                    Item::add_node(Some(entry.id.clone()), params);
                    return true;
                }
                false
            })
            .collect();
        !items.is_empty()
    }

    pub fn drop_all(&mut self, position: Vec2, include_credits: bool) {
        self.items.drain_filter(|entry| {
            if entry.params.is_quest_item == false {
                let params = ItemParams {
                    position: Some(Self::randomize_drop_position(position)),
                    ..entry.params.clone()
                };
                Item::add_node(Some(entry.id.clone()), params);
            }
            true
        });
        if include_credits {
            self.drop_all_credits(position);
        }
    }

    pub fn drop_credits(&mut self, amount: u32, position: Vec2) -> bool {
        if self.credits < amount {
            return false;
        }
        self.credits -= amount;
        Credits::add_node(Self::randomize_drop_position(position), amount);
        true
    }

    pub fn drop_all_credits(&mut self, position: Vec2) {
        self.drop_credits(self.credits, position);
    }

    pub fn get_total_weight(&self) -> f32 {
        let mut weight = 0.0;
        for item in &self.items {
            weight += item.params.weight;
        }
        weight
    }

    pub fn to_params(&self) -> ActorInventoryParams {
        ActorInventoryParams {
            items: self.items.iter().map( | entry| entry.id.clone()).collect(),
            credits: self.credits,
        }
    }

    fn randomize_drop_position(position: Vec2) -> Vec2 {
        vec2(
            rand::gen_range(position.x - Self::DROP_ALL_POSITION_VARIANCE, position.x + Self::DROP_ALL_POSITION_VARIANCE),
            rand::gen_range(position.y - Self::DROP_ALL_POSITION_VARIANCE, position.y + Self::DROP_ALL_POSITION_VARIANCE),
        )
    }
}

impl From<ActorInventoryParams> for ActorInventory {
    fn from(params: ActorInventoryParams) -> Self {
        let resources = storage::get::<Resources>();
        ActorInventory {
            items: params.items.into_iter().map(|id| {
                let params = resources.items.get(&id).cloned().unwrap();
                ActorInventoryEntry::new(None, params)
            }).collect(),
            credits: params.credits,
        }
    }
}
