use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(remote = "Color")]
pub struct ColorDef {
    #[serde(rename = "red", alias ="r")]
    pub r: f32,
    #[serde(rename = "green", alias ="g")]
    pub g: f32,
    #[serde(rename = "blue", alias ="b")]
    pub b: f32,
    #[serde(rename = "alpha", alias ="a")]
    pub a: f32,
}

impl From<Color> for ColorDef {
    fn from(other: Color) -> Self {
        ColorDef {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

impl From<ColorDef> for Color {
    fn from(other: ColorDef) -> Self {
        Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

pub mod opt_color {
    use super::{Color};
    use serde::{Serialize, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(value: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "super::ColorDef")] &'a Color);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "super::ColorDef")] Color);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(remote = "Animation")]
pub struct AnimationDef {
    pub name: String,
    pub row: u32,
    pub frames: u32,
    pub fps: u32,
}

impl From<&Animation> for AnimationDef {
    fn from(other: &Animation) -> Self {
        AnimationDef {
            name: other.name.clone(),
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

impl From<AnimationDef> for Animation {
    fn from(other: AnimationDef) -> Self {
        Animation {
            name: other.name,
            row: other.row,
            frames: other.frames,
            fps: other.fps,
        }
    }
}

pub mod vec_animation {
    use super::{Animation, AnimationDef};
    use serde::{Serialize, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(value: &Vec<Animation>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "AnimationDef")] &'a Animation);

        value
            .into_iter()
            .map(Helper)
            .collect::<Vec<Helper>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Animation>, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "AnimationDef")] Animation);

        let helper = Vec::deserialize(deserializer)?;
        Ok(helper.iter().map(|Helper(external)| external.clone()).collect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FilterMode")]
pub enum FilterModeDef {
    #[serde(alias = "linear")]
    Linear,
    #[serde(alias = "nearest_neighbor")]
    Nearest,
}
