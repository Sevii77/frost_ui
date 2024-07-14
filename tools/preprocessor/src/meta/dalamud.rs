use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OptionOrStatic<T: OptionValue> {
	OptionSub(String, HashMap<String, T::Value>),
	Option(String),
	OptionMul(String, T::Value),
	Static(T::Value),
}

pub trait OptionValue {
	type Value: Clone;
}

impl OptionValue for i32 {
	type Value = Self;
}

impl OptionValue for f32 {
	type Value = Self;
}

impl OptionValue for [f32; 2] {
	type Value = Self;
}

impl OptionValue for [f32; 4] {
	type Value = Self;
}

// ----------

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct Style {
	pub alpha: OptionOrStatic<f32>,
	pub window_padding: OptionOrStatic<[f32; 2]>,
	pub window_rounding: OptionOrStatic<f32>,
	pub window_border_size: OptionOrStatic<f32>,
	pub window_title_align: OptionOrStatic<[f32; 2]>,
	pub window_menu_button_position: OptionOrStatic<i32>,
	pub child_rounding: OptionOrStatic<f32>,
	pub child_border_size: OptionOrStatic<f32>,
	pub popup_rounding: OptionOrStatic<f32>,
	pub popup_border_size: OptionOrStatic<f32>,
	pub frame_padding: OptionOrStatic<[f32; 2]>,
	pub frame_rounding: OptionOrStatic<f32>,
	pub frame_border_size: OptionOrStatic<f32>,
	pub item_spacing: OptionOrStatic<[f32; 2]>,
	pub item_inner_spacing: OptionOrStatic<[f32; 2]>,
	pub cell_padding: OptionOrStatic<[f32; 2]>,
	pub touch_extra_padding: OptionOrStatic<[f32; 2]>,
	pub indent_spacing: OptionOrStatic<f32>,
	pub scrollbar_size: OptionOrStatic<f32>,
	pub scrollbar_rounding: OptionOrStatic<f32>,
	pub grab_min_size: OptionOrStatic<f32>,
	pub grab_rounding: OptionOrStatic<f32>,
	pub log_slider_deadzone: OptionOrStatic<f32>,
	pub tab_rounding: OptionOrStatic<f32>,
	pub tab_border_size: OptionOrStatic<f32>,
	pub button_text_align: OptionOrStatic<[f32; 2]>,
	pub selectable_text_align: OptionOrStatic<[f32; 2]>,
	pub display_safe_area_padding: OptionOrStatic<[f32; 2]>,
	pub colors: HashMap<String, OptionOrStatic<[f32; 4]>>,
}

impl Default for Style {
	fn default() -> Self {
		Self {
			alpha: OptionOrStatic::Static(1.0),
			window_padding: OptionOrStatic::Static([8.0, 8.0]),
			window_rounding: OptionOrStatic::Static(4.0),
			window_border_size: OptionOrStatic::Static(0.0),
			window_title_align: OptionOrStatic::Static([0.0, 0.5]),
			window_menu_button_position: OptionOrStatic::Static(1),
			child_rounding: OptionOrStatic::Static(0.0),
			child_border_size: OptionOrStatic::Static(1.0),
			popup_rounding: OptionOrStatic::Static(0.0),
			popup_border_size: OptionOrStatic::Static(0.0),
			frame_padding: OptionOrStatic::Static([4.0, 3.0]),
			frame_rounding: OptionOrStatic::Static(4.0),
			frame_border_size: OptionOrStatic::Static(0.0),
			item_spacing: OptionOrStatic::Static([8.0, 4.0]),
			item_inner_spacing: OptionOrStatic::Static([4.0, 4.0]),
			cell_padding: OptionOrStatic::Static([4.0, 2.0]),
			touch_extra_padding: OptionOrStatic::Static([0.0, 0.0]),
			indent_spacing: OptionOrStatic::Static(21.0),
			scrollbar_size: OptionOrStatic::Static(16.0),
			scrollbar_rounding: OptionOrStatic::Static(9.0),
			grab_min_size: OptionOrStatic::Static(13.0),
			grab_rounding: OptionOrStatic::Static(3.0),
			log_slider_deadzone: OptionOrStatic::Static(4.0),
			tab_rounding: OptionOrStatic::Static(4.0),
			tab_border_size: OptionOrStatic::Static(0.0),
			button_text_align: OptionOrStatic::Static([0.5, 0.5]),
			selectable_text_align: OptionOrStatic::Static([0.0, 0.0]),
			display_safe_area_padding: OptionOrStatic::Static([3.0, 3.0]),
			colors: HashMap::new(),
		}
	}
}