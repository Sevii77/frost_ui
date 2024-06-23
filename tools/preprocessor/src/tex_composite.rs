// Stripped down version of https://github.com/Sevii77/ffxiv_materialui_accent/blob/aetherment/aetherment/src/modman/composite/tex.rs
// only used for the json structure
// TODO: the v1.5 will use a version of the above anyways, use it as a dependency at that point

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Path {
	Mod(String),
	Game(String),
	Option(String, String),
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tex {
	pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Layer {
	pub name: String,
	pub path: Path,
	pub modifiers: Vec<Modifier>,
	pub blend: Blend,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Blend {
	Normal,
	Multiply,
	Screen,
	Overlay,
	HardLight,
	SoftLightPhotoshop,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Modifier {
	/// Culls pixels based on the red channel of the mask texture.
	AlphaMask {
		path: Path,
		cull_point: OptionOrStatic<MaskOption>,
	},
	
	/// Culls pixels based on the red channel of the mask texture, then stretches the alpha channel of the texture.
	AlphaMaskAlphaStretch {
		path: Path,
		cull_point: OptionOrStatic<MaskOption>,
	},
	
	/// Multiplies the color channels of the texture by the color.
	Color {
		value: OptionOrStatic<ColorOption>,
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum OptionOrStatic<T: OptionSetting + Sized + Default> {
	Option(T),
	Static(T::Value),
}

pub trait OptionSetting {
	type Value: Clone + Default + PartialEq;
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct ColorOption(pub String);
impl OptionSetting for ColorOption {
	type Value = [f32; 4];
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct MaskOption(pub String);
impl OptionSetting for MaskOption {
	type Value = f32;
}