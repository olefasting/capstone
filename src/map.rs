use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, SmallVec, Point, DistanceAlg, a_star_search};

use crate::prelude::*;

use crate::json::{
    TiledMap,
};
use std::path::Path;

pub const MAP_LAYER_GROUND: &'static str = "ground";
pub const MAP_LAYER_SOLIDS: &'static str = "solids";
pub const MAP_LAYER_BARRIERS: &'static str = "barriers";
pub const MAP_LAYER_ITEMS: &'static str = "items";
pub const MAP_LAYER_SPAWN_POINTS: &'static str = "spawn_points";
pub const MAP_LAYER_NAVIGATION: &'static str = "navigation";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapLayerKind {
    #[serde(rename = "tile_layer")]
    TileLayer,
    #[serde(rename = "object_layer")]
    ObjectLayer,
}

impl Default for MapLayerKind {
    fn default() -> Self {
        MapLayerKind::TileLayer
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayer {
    pub id: String,
    pub kind: MapLayerKind,
    #[serde(default, skip_serializing_if = "CollisionKind::is_none")]
    pub collision: CollisionKind,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    pub tiles: Vec<Option<MapTile>>,
    pub objects: Vec<MapObject>,
    #[serde(default)]
    pub is_visible: bool,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, MapProperty>,
}

impl Default for MapLayer {
    fn default() -> Self {
        MapLayer {
            id: "".to_string(),
            collision: CollisionKind::None,
            kind: MapLayerKind::TileLayer,
            grid_size: UVec2::ZERO,
            tiles: Vec::new(),
            objects: Vec::new(),
            is_visible: true,
            properties: HashMap::new(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapTile {
    pub tile_id: u32,
    pub tileset_id: String,
    pub texture_id: String,
    #[serde(with = "json::def_vec2")]
    pub texture_coords: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    pub name: String,
    #[serde(with = "json::def_vec2")]
    pub position: Vec2,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec2>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, MapProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapPropertyType {
    /* unsupported:
    #[serde(rename = "bool")]
    BoolType,
    #[serde(rename = "float")]
    FloatType,
    #[serde(rename = "integer")]
    IntType,
    #[serde(rename = "object")]
    ObjectType,
     */
    #[serde(rename = "string")]
    StringType,
    #[serde(rename = "color")]
    ColorType,
    #[serde(rename = "file")]
    FileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapProperty {
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: MapPropertyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapTileset {
    pub id: String,
    pub texture_id: String,
    #[serde(with = "json::def_uvec2")]
    pub texture_size: UVec2,
    #[serde(with = "json::def_uvec2")]
    pub tile_size: UVec2,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    pub first_tile_id: u32,
    pub tile_cnt: u32,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, MapProperty>,
}

impl MapTileset {
    pub fn get_texture_coords(&self, tile_id: u32) -> Vec2 {
        let x = ((tile_id % self.grid_size.x) * self.tile_size.x) as f32;
        let y = ((tile_id / self.grid_size.x) * self.tile_size.y) as f32;
        vec2(x, y)
    }
}

impl From<String> for CollisionKind {
    fn from(str: String) -> Self {
        if str == "barrier".to_string() {
            CollisionKind::Barrier
        } else if str == "solid".to_string() {
            CollisionKind::Solid
        } else {
            CollisionKind::None
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "json::MapDef", from = "json::MapDef")]
pub struct Map {
    pub id: String,
    #[serde(default = "Map::default_background_color", with = "json::ColorDef")]
    pub background_color: Color,
    #[serde(with = "json::def_vec2")]
    pub world_offset: Vec2,
    #[serde(with = "json::def_uvec2")]
    pub grid_size: UVec2,
    #[serde(with = "json::def_vec2")]
    pub tile_size: Vec2,
    pub layers: HashMap<String, MapLayer>,
    pub tilesets: HashMap<String, MapTileset>,
    #[serde(skip)]
    pub draw_order: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, MapProperty>,
}

impl Map {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self, FileError> {
        let bytes = load_file(path.as_ref().to_str().unwrap()).await?;
        let map = serde_json::from_slice(&bytes).unwrap();
        Ok(map)
    }

    pub async fn load_tiled<P: AsRef<Path>>(id: &str, path: P, export_path: Option<P>) -> Result<Self, FileError> {
        let bytes = load_file(path.as_ref().to_str().unwrap()).await?;
        let tiled_map: TiledMap = serde_json::from_slice(&bytes).unwrap();
        let map = Map::from_tiled(id, tiled_map);

        if let Some(export_path) = export_path {
            map.save(export_path).unwrap();
        }

        Ok(map)
    }

    pub fn to_grid(&self, rect: Rect) -> URect {
        let p = self.to_coords(rect.point());
        let w = ((rect.w / self.tile_size.x) as u32).clamp(0, self.grid_size.x - p.x - 1);
        let h = ((rect.h / self.tile_size.y) as u32).clamp(0, self.grid_size.y - p.y - 1);
        URect::new(p.x, p.y, w, h)
    }

    pub fn to_coords(&self, position: Vec2) -> UVec2 {
        let x = (((position.x - self.world_offset.x) / self.tile_size.x) as u32).clamp(0, self.grid_size.x - 1);
        let y = (((position.y - self.world_offset.y) / self.tile_size.y) as u32).clamp(0, self.grid_size.y - 1);
        uvec2(x, y)
    }

    pub fn to_position(&self, point: UVec2) -> Vec2 {
        vec2(
            point.x as f32 * self.tile_size.x + self.world_offset.x,
            point.y as f32 * self.tile_size.y + self.world_offset.y,
        )
    }
    pub fn get_tile(&self, layer_id: &str, x: u32, y: u32) -> &Option<MapTile> {
        let layer = self.layers
            .get(layer_id)
            .expect(&format!("No layer with id '{}'!", layer_id));

        if x >= self.grid_size.x || y >= self.grid_size.y {
            return &None;
        };

        let i = (y * self.grid_size.x + x) as usize;
        &layer.tiles[i]
    }

    pub fn get_tiles(&self, layer_id: &str, rect: Option<URect>) -> MapTileIterator {
        let rect = rect.unwrap_or(URect::new(0, 0, self.grid_size.x, self.grid_size.y));
        let layer = self.layers.get(layer_id)
            .expect(&format!("No layer with id '{}'!", layer_id));

        MapTileIterator::new(layer, rect)
    }

    pub fn get_path(&self, start: Vec2, end: Vec2) -> Option<NavigationPath> {
        let p1 = self.to_coords(start);
        let p2 = self.to_coords(end);

        let mut path = a_star_search(
            self.point2d_to_index(Point::new(p1.x, p1.y)),
            self.point2d_to_index(Point::new(p2.x, p2.y)),
            self,
        );

        if path.success {
            path.steps.remove(0);

            let p = self.index_to_point2d(path.destination);
            let destination = self.to_position(uvec2(p.x as u32, p.y as u32));
            let nodes = path.steps
                .into_iter()
                .map(|idx| {
                    let p = self.index_to_point2d(idx);
                    self.to_position(uvec2(p.x as u32, p.y as u32)) + self.tile_size / 2.0
                })
                .collect();

            let path = NavigationPath {
                destination,
                nodes,
            };

            return Some(path);
        }

        None
    }

    pub fn get_collisions(&self, collider: Collider) -> Vec<(Vec2, CollisionKind)> {
        let rect = self.to_grid(collider.with_padding(self.tile_size.x * 2.0).into());
        let mut collisions = Vec::new();
        for (_, layer) in &self.layers {
            if layer.is_visible {
                if layer.collision != CollisionKind::None {
                    for (x, y, tile) in self.get_tiles(&layer.id, Some(rect)) {
                        if let Some(_) = tile {
                            let tile_position = self.to_position(uvec2(x, y));
                            if Collider::rect(
                                tile_position.x,
                                tile_position.y,
                                self.tile_size.x,
                                self.tile_size.y,
                            ).overlaps(collider) {
                                collisions.push((
                                    tile_position,
                                    layer.collision.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        collisions
    }

    pub fn draw(&self, rect: Option<URect>) {
        let rect = rect.unwrap_or(URect::new(0, 0, self.grid_size.x, self.grid_size.y));
        draw_rectangle(
            self.world_offset.x + (rect.x as f32 * self.tile_size.x),
            self.world_offset.y + (rect.y as f32 * self.tile_size.y),
            rect.w as f32 * self.tile_size.x,
            rect.h as f32 * self.tile_size.y,
            self.background_color,
        );

        let resources = storage::get::<Resources>();
        for layer_id in &self.draw_order {
            if let Some(layer) = self.layers.get(layer_id) {
                if layer.is_visible {
                    match layer.kind {
                        MapLayerKind::TileLayer => {
                            for (x, y, tile) in self.get_tiles(layer_id, Some(rect)) {
                                if let Some(tile) = tile {
                                    let world_position = self.world_offset + vec2(
                                        x as f32 * self.tile_size.x,
                                        y as f32 * self.tile_size.y,
                                    );

                                    let texture = resources.textures
                                        .get(&tile.texture_id)
                                        .cloned()
                                        .expect(&format!("No texture with id '{}'!", tile.texture_id));
                                    draw_texture_ex(
                                        texture,
                                        world_position.x,
                                        world_position.y,
                                        color::WHITE,
                                        DrawTextureParams {
                                            source: Some(Rect::new(
                                                tile.texture_coords.x, // + 0.1,
                                                tile.texture_coords.y, // + 0.1,
                                                self.tile_size.x, // - 0.2,
                                                self.tile_size.y, // - 0.2,
                                            )),
                                            dest_size: Some(vec2(
                                                self.tile_size.x,
                                                self.tile_size.y,
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn default_background_color() -> Color {
        color::BLACK
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    #[cfg(target_family = "wasm")]
    pub fn save<P: AsRef<Path>>(&self, _: P) -> io::Result<()> {
        Ok(())
    }

    pub fn from_tiled(id: &str, tiled_map: TiledMap) -> Self {
        let background_color = if let Some(background_color) = tiled_map.backgroundcolor {
            color_from_hex_string(&background_color)
        } else {
            color::BLACK
        };

        let mut tilesets = HashMap::new();
        for tiled_tileset in tiled_map.tilesets {
            let texture_id = tiled_tileset.properties.clone()
                .expect(&format!("Tiled tileset '{}' needs a 'texture_id' property!", tiled_tileset.name))
                .into_iter()
                .find(|prop| prop.name == "texture_id")
                .expect(&format!("Tiled tileset '{}' needs a 'texture_id' property!", tiled_tileset.name))
                .value;

            let texture_size = uvec2(tiled_tileset.imagewidth as u32, tiled_tileset.imageheight as u32);
            let tile_size = uvec2(tiled_tileset.tilewidth as u32, tiled_tileset.tileheight as u32);
            let grid_size = uvec2(tiled_tileset.columns as u32, tiled_tileset.tilecount as u32 / tiled_tileset.columns as u32);

            let mut properties = HashMap::new();
            if let Some(tiled_props) = tiled_tileset.properties {
                for tiled_property in tiled_props {
                    let value_type = tiled_property.value_type.into();

                    let property = MapProperty {
                        value: tiled_property.value,
                        value_type,
                    };

                    properties.insert(tiled_property.name, property);
                }
            }

            let tileset = MapTileset {
                id: tiled_tileset.name.clone(),
                texture_id: texture_id.to_string(),
                texture_size,
                tile_size,
                grid_size,
                first_tile_id: tiled_tileset.firstgid,
                tile_cnt: tiled_tileset.tilecount,
                properties,
            };

            tilesets.insert(tiled_tileset.name, tileset);
        }

        let mut layers = HashMap::new();
        let mut draw_order = Vec::new();
        for tiled_layer in &tiled_map.layers {
            let collision = match &tiled_layer.properties {
                Some(props) => match props.into_iter().find(|prop| prop.name == "collision".to_string()) {
                    Some(collision) => CollisionKind::from(collision.value.clone()),
                    _ => CollisionKind::None,
                },
                _ => CollisionKind::None,
            };

            let mut tiles = Vec::new();
            for tile_id in tiled_layer.data.clone() {
                let res = if tile_id != 0 {
                    let tileset = tilesets
                        .iter()
                        .find_map(|(_, tileset)| {
                            if tile_id >= tileset.first_tile_id
                                && tile_id <= tileset.first_tile_id + tileset.tile_cnt {
                                return Some(tileset);
                            }
                            None
                        })
                        .unwrap();

                    let tile_id = tile_id - tileset.first_tile_id;
                    let tile = MapTile {
                        tile_id,
                        tileset_id: tileset.id.clone(),
                        texture_id: tileset.texture_id.clone(),
                        texture_coords: tileset.get_texture_coords(tile_id),
                    };

                    Some(tile)
                } else {
                    None
                };

                tiles.push(res);
            }


            let mut objects = Vec::new();
            for object in &tiled_layer.objects {
                let position = vec2(object.x, object.y);
                let size = {
                    let size = vec2(object.width, object.height);
                    if size != Vec2::ZERO {
                        Some(size)
                    } else {
                        None
                    }
                };

                let mut properties = HashMap::new();
                if let Some(tiled_props) = object.properties.clone() {
                    for tiled_property in tiled_props {
                        let value_type = tiled_property.value_type;

                        let property = MapProperty {
                            value: tiled_property.value,
                            value_type,
                        };

                        properties.insert(tiled_property.name, property);
                    }
                }

                let object = MapObject {
                    name: object.name.clone(),
                    position,
                    size,
                    properties,
                };

                objects.push(object);
            }

            let kind = if tiled_layer.layer_type == "tilelayer".to_string() {
                MapLayerKind::TileLayer
            } else {
                MapLayerKind::ObjectLayer
            };

            let grid_size = uvec2(tiled_map.width, tiled_map.height);

            let mut properties = HashMap::new();
            if let Some(tiled_props) = &tiled_layer.properties {
                for tiled_property in tiled_props {
                    let value_type = tiled_property.value_type.clone();

                    let property = MapProperty {
                        value: tiled_property.value.clone(),
                        value_type,
                    };

                    properties.insert(tiled_property.name.clone(), property);
                }
            }

            let layer = MapLayer {
                id: tiled_layer.name.clone(),
                kind,
                collision,
                grid_size,
                tiles,
                objects,
                is_visible: tiled_layer.visible,
                properties,
            };

            draw_order.push(layer.id.clone());
            layers.insert(layer.id.clone(), layer);
        }

        let grid_size = uvec2(tiled_map.width, tiled_map.height);

        let mut properties = HashMap::new();
        if let Some(tiled_props) = tiled_map.properties {
            for tiled_property in tiled_props {
                let value_type = tiled_property.value_type.into();

                let property = MapProperty {
                    value: tiled_property.value,
                    value_type,
                };

                properties.insert(tiled_property.name, property);
            }
        }

        Map {
            id: id.to_string(),
            background_color,
            world_offset: Vec2::ZERO,
            grid_size,
            tile_size: vec2(tiled_map.tilewidth as f32, tiled_map.tileheight as f32),
            layers,
            tilesets,
            draw_order,
            properties,
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        let x = idx as u32 % self.grid_size.y;
        let y = idx as u32 / self.grid_size.y;
        for (layer_id, layer) in &self.layers {
            if layer.collision == CollisionKind::Solid && self.get_tile(layer_id, x, y).is_some() {
                return true;
            }
        }
        false
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let len = (self.grid_size.x * self.grid_size.y) as i32;

        let n = idx as i32 - self.grid_size.x as i32;
        let e = idx as i32 + 1;
        let s = idx as i32 + self.grid_size.x as i32;
        let w = idx as i32 - 1;
        let ne = n + 1;
        let se = s + 1;
        let sw = s - 1;
        let nw = n - 1;

        // N, NE, E, SE, S, SW, W, NW
        let mut exits = (
            n >= 0 && n < len,
            ne >= 0 && ne < len,
            e >= 0 && e < len,
            se >= 0 && se < len,
            s >= 0 && s < len,
            sw >= 0 && sw < len,
            w >= 0 && w < len,
            nw >= 0 && nw < len,
        );

        #[cfg(feature = "navigation_layers")]
        if let Some(layer) = self.layers.get(MAP_LAYER_NAVIGATION) {
            exits.0 = exits.0 == true && layer.tiles[n as usize].is_some();
            exits.2 = exits.2 == true && layer.tiles[e as usize].is_some();
            exits.4 = exits.4 == true && layer.tiles[s as usize].is_some();
            exits.6 = exits.6 == true && layer.tiles[w as usize].is_some();
            exits.1 = exits.1 == true && exits.0 == true && exits.2 == true && layer.tiles[ne as usize].is_some();
            exits.3 = exits.3 == true && exits.2 == true && exits.4 == true && layer.tiles[se as usize].is_some();
            exits.5 = exits.5 == true && exits.4 == true && exits.6 == true && layer.tiles[sw as usize].is_some();
            exits.7 = exits.7 == true && exits.6 == true && exits.0 == true && layer.tiles[nw as usize].is_some();
        }

        #[cfg(not(feature = "navigation_layers"))]
        for (_, layer) in &self.layers {
            match layer.collision {
                CollisionKind::None => continue,
                _ => {
                    if exits.0 == false || layer.tiles[n as usize].is_some() {
                        exits.0 = false;
                        exits.1 = false;
                        exits.7 = false;
                    }
                    if exits.1 == false || layer.tiles[ne as usize].is_some() {
                        exits.1 = false;
                    }
                    if exits.2 == false || layer.tiles[e as usize].is_some() {
                        exits.2 = false;
                        exits.1 = false;
                        exits.3 = false;
                    }
                    if exits.3 == false || layer.tiles[se as usize].is_some() {
                        exits.3 = false;
                    }
                    if exits.4 == false || layer.tiles[s as usize].is_some() {
                        exits.4 = false;
                        exits.3 = false;
                        exits.5 = false;
                    }
                    if exits.5 == false || layer.tiles[sw as usize].is_some() {
                        exits.5 = false;
                    }
                    if exits.6 == false || layer.tiles[w as usize].is_some() {
                        exits.6 = false;
                        exits.5 = false;
                        exits.7 = false;
                    }
                    if exits.7 == false || layer.tiles[nw as usize].is_some() {
                        exits.7 = false;
                    }
                    if exits == (false, false, false, false, false, false, false, false) {
                        break;
                    }
                }
            }
        }

        let mut res = SmallVec::new();
        if exits.0 { res.push((n as usize, 1.0)) }
        if exits.1 { res.push((ne as usize, 1.0)) }
        if exits.2 { res.push((e as usize, 1.0)) }
        if exits.3 { res.push((se as usize, 1.0)) }
        if exits.4 { res.push((s as usize, 1.0)) }
        if exits.5 { res.push((sw as usize, 1.0)) }
        if exits.6 { res.push((w as usize, 1.0)) }
        if exits.7 { res.push((nw as usize, 1.0)) }
        res
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = self.index_to_point2d(idx1);
        let p2 = self.index_to_point2d(idx2);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.grid_size.x, self.grid_size.y)
    }
}

pub struct MapTileIterator<'a> {
    rect: URect,
    current: (u32, u32),
    layer: &'a MapLayer,
}

impl<'a> MapTileIterator<'a> {
    fn new(layer: &'a MapLayer, rect: URect) -> Self {
        let current = (rect.x, rect.y);
        MapTileIterator {
            layer,
            rect,
            current,
        }
    }
}

impl<'a> Iterator for MapTileIterator<'a> {
    type Item = (u32, u32, &'a Option<MapTile>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.current.0 + 1 >= self.rect.x + self.rect.w {
            (self.rect.x, self.current.1 + 1)
        } else {
            (self.current.0 + 1, self.current.1)
        };

        if self.current.1 >= self.rect.y + self.rect.h {
            return None;
        }

        let i = (self.current.1 * self.layer.grid_size.x + self.current.0) as usize;
        let res = Some((
            self.current.0,
            self.current.1,
            &self.layer.tiles[i],
        ));
        self.current = next;
        return res;
    }
}

#[derive(Debug, Clone)]
pub struct NavigationPath {
    pub destination: Vec2,
    pub nodes: Vec<Vec2>,
}
