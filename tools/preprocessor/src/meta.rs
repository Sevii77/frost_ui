// Stripped down version of https://github.com/Sevii77/ffxiv_materialui_accent/blob/aetherment/aetherment/src/modman/meta.rs
// only used for the json structure
// TODO: the v1.5 will use a version of the above anyways, use it as a dependency at that point

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MetaBase {
	pub name: String,
	pub description: String,
	pub version: String,
	pub author: String,
	pub website: String,
	pub tags: Vec<String>,
	pub dependencies: Vec<String>,
	pub options: Vec<HashMap<String, OptionBase>>,
	pub presets: Vec<HashMap<String, HashMap<String, ValueBase>>>,
}

// #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum OptionBaseType {
// 	Category(String),
// 	Option(HashMap<String, OptionBase>),
// }

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OptionBase {
	Category(i32),
	Files(Vec<String>),
	Color(HashMap<String, Vec<f32>>),
	// Color {
	// 	default: Vec<f32>,
	// 	min: Vec<f32>,
	// 	max: Vec<f32>,
	// },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ValueBase {
	Files(String),
	Color(Vec<f32>),
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Meta {
	pub name: String,
	pub description: String,
	pub version: String,
	pub author: String,
	pub website: String,
	pub tags: Vec<String>,
	pub dependencies: Vec<String>,
	pub options: Vec<OptionType>,
	pub presets: Vec<Preset>,
	
	pub files: HashMap<String, String>,
	pub file_swaps: HashMap<String, String>,
	pub manipulations: Vec<i32>, // we dont care about it
}

impl Default for Meta {
	fn default() -> Self {
		Self {
			name: "New Mod".to_string(),
			description: String::new(),
			version: "1.0.0".to_string(),
			author: String::new(),
			website: String::new(),
			tags: Vec::new(),
			dependencies: Vec::new(),
			options: Vec::new(),
			presets: Vec::new(),
			
			files: HashMap::new(),
			file_swaps: HashMap::new(),
			manipulations: Vec::new(),
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Preset {
	pub name: String,
	pub settings: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Value {
	SingleFiles(u32),
	MultiFiles(u32),
	Rgb([f32; 3]),
	Rgba([f32; 4]),
	Grayscale(f32),
	Opacity(f32),
	Mask(f32),
	Path(u32),
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OptionType {
	Category(String),
	Option(Option),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Option {
	pub name: String,
	pub description: String,
	pub settings: OptionSettings,
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OptionSettings {
	SingleFiles(ValueFiles),
	MultiFiles(ValueFiles),
	Rgb(ValueRgb),
	Rgba(ValueRgba),
	Grayscale(ValueSingle),
	Opacity(ValueSingle),
	Mask(ValueSingle),
	Path(ValuePath),
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueFiles {
	pub default: u32,
	pub options: Vec<ValueFilesOption>,
}

impl Default for ValueFiles {
	fn default() -> Self {
		Self {
			default: 0,
			options: vec![],
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueFilesOption {
	pub name: String,
	pub description: String,
	pub inherit: std::option::Option<String>,
	pub files: HashMap<String, String>,
	pub file_swaps: HashMap<String, String>,
	pub manipulations: Vec<i32>,
}

impl Default for ValueFilesOption {
	fn default() -> Self {
		Self {
			name: "New sub option".to_owned(),
			description: String::new(),
			inherit: None,
			files: HashMap::new(),
			file_swaps: HashMap::new(),
			manipulations: Vec::new(),
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueRgb {
	pub default: [f32; 3],
	pub min: [f32; 3],
	pub max: [f32; 3],
}

impl Default for ValueRgb {
	fn default() -> Self {
		Self {
			default: [1.0, 1.0, 1.0],
			min: [0.0, 0.0, 0.0],
			max: [1.0, 1.0, 1.0],
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueRgba {
	pub default: [f32; 4],
	pub min: [f32; 4],
	pub max: [f32; 4],
}

impl Default for ValueRgba {
	fn default() -> Self {
		Self {
			default: [1.0, 1.0, 1.0, 1.0],
			min: [0.0, 0.0, 0.0, 0.0],
			max: [1.0, 1.0, 1.0, 1.0],
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValueSingle {
	pub default: f32,
	pub min: f32,
	pub max: f32,
}

impl Default for ValueSingle {
	fn default() -> Self {
		Self {
			default: 0.0,
			min: 0.0,
			max: 1.0,
		}
	}
}

// ----------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ValuePath {
	pub default: u32,
	pub options: Vec<(String, Vec<(String, crate::tex_composite::Path)>)>,
}

impl Default for ValuePath {
	fn default() -> Self {
		Self {
			default: 0,
			options: Vec::new(),
		}
	}
}