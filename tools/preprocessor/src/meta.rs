// Stripped down version of https://github.com/Sevii77/ffxiv_materialui_accent/blob/aetherment/aetherment/src/modman/meta.rs
// only used for the json structure
// TODO: the v1.5 will use a version of the above anyways, use it as a dependency at that point

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod dalamud;

#[derive(Debug, Clone, Deserialize, Serialize)]
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
	pub style: StyleBase,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ValueBase {
	Files(String),
	Color(Vec<f32>),
}

// ----------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StyleBase {
	pub variables: StyleVariables,
	pub colors: HashMap<String, OptionOrStaticBase<[f32; 4]>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct StyleVariables {
	pub alpha: OptionOrStaticBase<f32>,
	pub window_padding: OptionOrStaticBase<[f32; 2]>,
	pub window_rounding: OptionOrStaticBase<f32>,
	pub window_border_size: OptionOrStaticBase<f32>,
	pub window_title_align: OptionOrStaticBase<[f32; 2]>,
	pub window_menu_button_position: OptionOrStaticBase<i32>,
	pub child_rounding: OptionOrStaticBase<f32>,
	pub child_border_size: OptionOrStaticBase<f32>,
	pub popup_rounding: OptionOrStaticBase<f32>,
	pub popup_border_size: OptionOrStaticBase<f32>,
	pub frame_padding: OptionOrStaticBase<[f32; 2]>,
	pub frame_rounding: OptionOrStaticBase<f32>,
	pub frame_border_size: OptionOrStaticBase<f32>,
	pub item_spacing: OptionOrStaticBase<[f32; 2]>,
	pub item_inner_spacing: OptionOrStaticBase<[f32; 2]>,
	pub cell_padding: OptionOrStaticBase<[f32; 2]>,
	pub touch_extra_padding: OptionOrStaticBase<[f32; 2]>,
	pub indent_spacing: OptionOrStaticBase<f32>,
	pub scrollbar_size: OptionOrStaticBase<f32>,
	pub scrollbar_rounding: OptionOrStaticBase<f32>,
	pub grab_min_size: OptionOrStaticBase<f32>,
	pub grab_rounding: OptionOrStaticBase<f32>,
	pub log_slider_deadzone: OptionOrStaticBase<f32>,
	pub tab_rounding: OptionOrStaticBase<f32>,
	pub tab_border_size: OptionOrStaticBase<f32>,
	pub button_text_align: OptionOrStaticBase<[f32; 2]>,
	pub selectable_text_align: OptionOrStaticBase<[f32; 2]>,
	pub display_safe_area_padding: OptionOrStaticBase<[f32; 2]>,
}

impl Default for StyleVariables {
	fn default() -> Self {
		Self {
			alpha: OptionOrStaticBase::Static(1.0),
			window_padding: OptionOrStaticBase::Static([8.0, 8.0]),
			window_rounding: OptionOrStaticBase::Static(4.0),
			window_border_size: OptionOrStaticBase::Static(0.0),
			window_title_align: OptionOrStaticBase::Static([0.0, 0.5]),
			window_menu_button_position: OptionOrStaticBase::Static(1),
			child_rounding: OptionOrStaticBase::Static(0.0),
			child_border_size: OptionOrStaticBase::Static(1.0),
			popup_rounding: OptionOrStaticBase::Static(0.0),
			popup_border_size: OptionOrStaticBase::Static(0.0),
			frame_padding: OptionOrStaticBase::Static([4.0, 3.0]),
			frame_rounding: OptionOrStaticBase::Static(4.0),
			frame_border_size: OptionOrStaticBase::Static(0.0),
			item_spacing: OptionOrStaticBase::Static([8.0, 4.0]),
			item_inner_spacing: OptionOrStaticBase::Static([4.0, 4.0]),
			cell_padding: OptionOrStaticBase::Static([4.0, 2.0]),
			touch_extra_padding: OptionOrStaticBase::Static([0.0, 0.0]),
			indent_spacing: OptionOrStaticBase::Static(21.0),
			scrollbar_size: OptionOrStaticBase::Static(16.0),
			scrollbar_rounding: OptionOrStaticBase::Static(9.0),
			grab_min_size: OptionOrStaticBase::Static(13.0),
			grab_rounding: OptionOrStaticBase::Static(3.0),
			log_slider_deadzone: OptionOrStaticBase::Static(4.0),
			tab_rounding: OptionOrStaticBase::Static(4.0),
			tab_border_size: OptionOrStaticBase::Static(0.0),
			button_text_align: OptionOrStaticBase::Static([0.5, 0.5]),
			selectable_text_align: OptionOrStaticBase::Static([0.0, 0.0]),
			display_safe_area_padding: OptionOrStaticBase::Static([3.0, 3.0]),
		}
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OptionOrStaticBase<T: dalamud::OptionValue> {
	OptionSub(HashMap<String, HashMap<String, T::Value>>),
	Option(String),
	OptionMul(String, T::Value),
	Static(T::Value),
}

impl<T: dalamud::OptionValue> OptionOrStaticBase<T> {
	pub fn convert(self) -> dalamud::OptionOrStatic<T> {
		match self {
			OptionOrStaticBase::OptionSub(v) => dalamud::OptionOrStatic::OptionSub(v.keys().next().unwrap().to_owned(), v.values().next().unwrap().to_owned()),
			OptionOrStaticBase::Option(o) => dalamud::OptionOrStatic::Option(o),
			OptionOrStaticBase::OptionMul(o, v) => dalamud::OptionOrStatic::OptionMul(o, v),
			OptionOrStaticBase::Static(v) => dalamud::OptionOrStatic::Static(v),
		}
	}
}

// ----------

#[derive(Debug, Clone, Deserialize, Serialize)]
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
	
	pub plugin_settings: PluginSettings,
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
			
			plugin_settings: PluginSettings::default(),
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct PluginSettings {
	pub dalamud: std::option::Option<dalamud::Style>,
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