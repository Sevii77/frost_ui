use std::{collections::HashMap, fs::File, io::{BufWriter, Cursor, Write}, path::{Path, PathBuf}};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// mod meta;
// mod tex_composite;
mod metabase;
mod icons;
mod uld;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
	let args = std::env::args().collect::<Vec<String>>();
	if args.len() < 3 {
		println!("Usage: {} <svg_root_dir> <target_dir> <meta.yaml>? <merge_dir>?", std::env::current_exe().unwrap().file_name().unwrap().to_string_lossy());
		return Ok(());
	}
	
	let mut files = HashMap::<Option<(String, String)>, HashMap<String, String>>::new();
	
	// svg renderer
	let svg_root = Path::new(&args[1]);
	let target_root = Path::new(&args[2]);
	println!("{:?}", target_root);
	
	let mut font = resvg::usvg::fontdb::Database::new();
	font.load_system_fonts();
	font.load_font_data(include_bytes!("Axis Extrabold.otf").to_vec());
	font.load_font_data(include_bytes!("Miedinger Bold.otf").to_vec());
	
	// let mut color_paths: HashMap<String, HashSet<String>> = HashMap::new();
	
	// fn walk_dir(path: &Path, target: &Path, files: &mut HashMap<Option<(String, String)>, HashMap<String, String>>, font: &resvg::usvg::fontdb::Database, color_paths: &mut HashMap<String, HashSet<String>>) -> Result<(), Error> {
	// 	for entry in std::fs::read_dir(path)? {
	// 		let entry_path = entry?.path();
	// 		if entry_path.is_dir() {
	// 			walk_dir(&entry_path, target, files, font, color_paths)?;
	// 		} else if entry_path.extension().map(|v| v.to_str()) == Some(Some("svg")) {
	// 			let svgs = split_svgs(&std::fs::read_to_string(entry_path)?)?;
	// 			for svg in svgs {
	// 				let local_dir = if let Some((o1, o2)) = &svg.option {
	// 					format!("{}/{o1}/{o2}", svg.path.clone())
	// 				} else {
	// 					svg.path.clone()
	// 				};
	// 				
	// 				let paths = files.entry(svg.option.clone()).or_insert_with(|| HashMap::new());
	// 				if svg.layers.len() > 1 || svg.layers[0].0 != None {
	// 					paths.insert(format!("{}.comp", &svg.path), format!("{local_dir}/comp.tex.comp"));
	// 					// if svg.path.starts_with("ui/uld/") {
	// 					// 	paths.insert(format!("{}.comp", &svg.path.replace("ui/uld/", "ui/uld/light/")), format!("{local_dir}/comp.tex.comp"));
	// 					// }
	// 				} else {
	// 					paths.insert(svg.path.clone(), format!("{local_dir}/0.tex"));
	// 					// if svg.path.starts_with("ui/uld/") {
	// 					// 	paths.insert(svg.path.replace("ui/uld/", "ui/uld/light/"), format!("{local_dir}/0.tex"));
	// 					// }
	// 				}
	// 				
	// 				render_svg(svg, target, font, color_paths)?;
	// 			}
	// 		}
	// 	}
	// 	
	// 	Ok(())
	// }
	
	// walk_dir(svg_root, &target_root.join("files"), &mut files, &font, &mut color_paths)?;
	
	fn get_svgs(path: &Path) -> Result<Vec<PathBuf>, Error> {
		let mut files = Vec::new();
		for entry in std::fs::read_dir(path)? {
			let entry_path = entry?.path();
			if entry_path.is_dir() {
				files.append(&mut get_svgs(&entry_path)?);
			} else if entry_path.extension().map(|v| v.to_str()) == Some(Some("svg")) {
				files.push(entry_path);
			}
		}
		
		Ok(files)
	}
	
	for a in get_svgs(svg_root)?.into_par_iter().map(|path| {
		let mut files = HashMap::new();
		let svgs = split_svgs(&std::fs::read_to_string(path).unwrap()).unwrap();
		for svg in svgs {
			let local_dir = if let Some((o1, o2)) = &svg.option {
				format!("{}/{o1}/{o2}", svg.path.clone())
			} else {
				svg.path.clone()
			};
			
			let paths = files.entry(svg.option.clone()).or_insert_with(|| HashMap::new());
			if svg.layers.len() > 1 || svg.layers[0].0 != None {
				paths.insert(format!("{}.comp", &svg.path), format!("{local_dir}/comp.tex.comp"));
				// if svg.path.starts_with("ui/uld/") {
				// 	paths.insert(format!("{}.comp", &svg.path.replace("ui/uld/", "ui/uld/fourth/")), format!("{local_dir}/comp.tex.comp"));
				// }
			} else {
				paths.insert(svg.path.clone(), format!("{local_dir}/0.tex"));
				// if svg.path.starts_with("ui/uld/") {
				// 	paths.insert(svg.path.replace("ui/uld/", "ui/uld/fourth/"), format!("{local_dir}/0.tex"));
				// }
			}
			
			render_svg(svg, &target_root.join("files"), &font).unwrap();
			// render_svg(svg, &target_root.join("files"), &font, &mut color_paths).unwrap();
		}
		
		files
	}).collect::<Vec<_>>() {
		for (k, v) in a {
			let b = files.entry(k).or_insert_with(|| HashMap::new());
			for (g, r) in v {
				b.insert(g, r);
			}
		}
	}
	
	// uld
	let entry = files.entry(None).or_insert_with(|| HashMap::new());
	for a in uld::ulds(&target_root)? {
		entry.insert(a.clone(), a);
	}
	
	// std::process::exit(0);
	
	// icons
	for ((o, so), f) in icons::job_icons(&target_root)? {
		let paths = files.entry(Some((o.to_owned(), so.to_owned()))).or_insert_with(|| HashMap::new());
		for (a, b) in f {
			paths.insert(a, b);
		}
	}
	
	let entry = files.entry(None).or_insert_with(|| HashMap::new());
	for (a, b) in icons::tribe_icons(&target_root)? {
		entry.insert(a, b);
	}
	
	let entry = files.entry(None).or_insert_with(|| HashMap::new());
	for (a, b) in icons::silver_bordered(&target_root)? {
		entry.insert(a, b);
	}
	
	let entry = files.entry(None).or_insert_with(|| HashMap::new());
	for (a, b) in icons::shop_icons(&target_root)? {
		entry.insert(a, b);
	}
	
	let entry = files.entry(None).or_insert_with(|| HashMap::new());
	for (a, b) in icons::menu_icons(&target_root)? {
		entry.insert(a, b);
	}
	
	// raw static files
	if args.len() >= 5 {
		fn walk_dir2(path: &Path, path_rel: String, target: &Path, files: &mut HashMap<Option<(String, String)>, HashMap<String, String>>) -> Result<(), Error> {
			for entry in std::fs::read_dir(path)? {
				let entry_path = entry?.path();
				let filename = entry_path.file_name().unwrap().to_string_lossy().to_string();
				if entry_path.is_dir() {
					if filename.contains(".") {
						for opt_entry in std::fs::read_dir(entry_path)? {
							let opt_entry_path = opt_entry?.path();
							let opt_name = opt_entry_path.file_name().unwrap().to_string_lossy().to_string();
							for sub_entry in std::fs::read_dir(opt_entry_path)? {
								let sub_entry_path = sub_entry?.path();
								let sub_name = sub_entry_path.file_name().unwrap().to_string_lossy().to_string();
								let paths = files.entry(Some((opt_name.to_string(), sub_name.to_string()))).or_insert_with(|| HashMap::new());
								
								for file_entry in std::fs::read_dir(sub_entry_path)? {
									let file_entry_path = file_entry?.path();
									let filename = file_entry_path.file_name().unwrap().to_string_lossy().to_string();
									
									let new_dir = target.join(&path_rel).join(&filename).join(&opt_name).join(&sub_name);
									_ = std::fs::create_dir_all(&new_dir);
									std::fs::copy(file_entry_path, new_dir.join(&filename))?;
									paths.insert(format!("{path_rel}{filename}"), format!("{path_rel}{filename}/{opt_name}/{sub_name}/{filename}"));
								}
							}
						}
					} else {
						walk_dir2(&entry_path, format!("{path_rel}{}/", filename), target, files)?;
					}
				} else {
					_ = std::fs::create_dir_all(&target.join(&path_rel));
					std::fs::copy(entry_path, target.join(&path_rel).join(&filename))?;
					let paths = files.entry(None).or_insert_with(|| HashMap::new());
					let path = format!("{path_rel}{filename}");
					paths.insert(path.clone(), path);
				}
			}
			
			Ok(())
		}
		
		walk_dir2(Path::new(&Path::new(&args[4])), String::new(), &target_root.join("files"), &mut files)?;
	}
	
	// meta file creation
	use aetherment::modman::{meta, issue::Issue, settings::*};
	
	if args.len() >= 4 {
		let meta_base = serde_yaml::from_slice::<metabase::MetaBase>(&std::fs::read(&args[3])?)?;
		
		let mut option_indexes = HashMap::new();
		let options = meta_base.options.into_iter().map(|o| {
			let mut key = o.keys().next().unwrap().split(";");
			let name = key.next().unwrap();
			let default = key.next();
			let value = o.values().next().unwrap();
			
			match value {
				metabase::OptionBase::Category(_) => meta::OptionType::Category(name.to_owned()),
				
				metabase::OptionBase::Files(sub_options) => {
					option_indexes.insert(name.to_owned(), sub_options.iter().enumerate().map(|(i, v)| (v.split(";").next().unwrap().to_owned(), i)).collect::<HashMap<_, _>>());
					
					meta::OptionType::Option(meta::Option {
						name: name.to_owned(),
						description: String::new(),
						settings: meta::OptionSettings::SingleFiles(meta::ValueFiles {
							default: default.map_or(0, |v| sub_options.iter().position(|v2| v2.split(";").next().unwrap() == v).map_or(0, |v| v as u32)),
							options: sub_options.into_iter().map(|sub_option| {
								let mut sub_option_segs = sub_option.split(";");
								let sub_option = sub_option_segs.next().unwrap();
								let inherit = sub_option_segs.next();
								
								let key = Some((name.to_owned(), sub_option.to_owned()));
								if !files.contains_key(&key) {
									println!("No files exist with option {name}:{sub_option}");
									std::process::exit(0);
								}
								
								meta::ValueFilesOption {
									name: sub_option.to_owned(),
									description: String::new(),
									inherit: inherit.map(|v| v.to_owned()),
									files: files[&key].clone(),
									
									..Default::default()
								}
							}).collect(),
						})
					})
				}
				
				metabase::OptionBase::Color(color) => {
					let default = &color["default"];
					let min = &color["min"];
					let max = &color["max"];
					
					meta::OptionType::Option(meta::Option {
						name: name.to_owned(),
						description: String::new(),
						settings: match default.len() {
							4 => meta::OptionSettings::Rgba(meta::ValueRgba {
								default: default[..].try_into().unwrap(),
								min: min[..].try_into().unwrap(),
								max: max[..].try_into().unwrap(),
							}),
							
							3 => meta::OptionSettings::Rgb(meta::ValueRgb {
								default: default[..].try_into().unwrap(),
								min: min[..].try_into().unwrap(),
								max: max[..].try_into().unwrap(),
							}),
							
							1 => meta::OptionSettings::Grayscale(meta::ValueSingle {
								default: default[0],
								min: min[0],
								max: max[0],
							}),
							
							_ => panic!("Unsupported color type")
						}
					})
				}
			}
		}).collect();
		
		let meta = meta::Meta {
			name: meta_base.name,
			description: meta_base.description,
			version: meta_base.version,
			author: meta_base.author,
			website: meta_base.website,
			tags: meta_base.tags,
			dependencies: meta_base.dependencies,
			
			presets: meta_base.presets.into_iter().map(|p| {
				Preset {
					name: p.keys().next().unwrap().to_owned(),
					settings: p.values().next().unwrap().into_iter().map(|(o, v)| (o.to_owned(), match v {
						metabase::ValueBase::Files(v) => Value::SingleFiles(option_indexes[o][v] as u32),
						metabase::ValueBase::Color(v) => match v.len() {
							4 => Value::Rgba(v[..].try_into().unwrap()),
							3 => Value::Rgb(v[..].try_into().unwrap()),
							_ => panic!("Unsupported color type")
						},
					})).collect(),
				}
			}).collect(),
			
			options: meta::Options(options),
			
			files: files.get(&None).map_or_else(|| HashMap::new(), |v| v.clone()),
			
			// ui_colors: meta_base.colors.into_iter().map(|p| meta::UiColor {
			// 	use_theme: true,
			// 	index: *p.keys().next().unwrap(),
			// 	color: p.values().next().unwrap().to_owned().convert(),
			// }).collect::<Vec<_>>(),
			ui_colors: meta_base.colors.into_iter().map(|(index, color)| meta::UiColor {
				use_theme: index < 1000,
				index,
				color: color.convert(),
			}).collect::<Vec<_>>(),
			
			plugin_settings: meta::PluginSettings {
				dalamud: Some({
					let s = meta_base.style.variables;
					meta::dalamud::Style {
						alpha: s.alpha.convert(),
						window_padding: s.window_padding.convert(),
						window_rounding: s.window_rounding.convert(),
						window_border_size: s.window_border_size.convert(),
						window_title_align: s.window_title_align.convert(),
						window_menu_button_position: s.window_menu_button_position.convert(),
						child_rounding: s.child_rounding.convert(),
						child_border_size: s.child_border_size.convert(),
						popup_rounding: s.popup_rounding.convert(),
						popup_border_size: s.popup_border_size.convert(),
						frame_padding: s.frame_padding.convert(),
						frame_rounding: s.frame_rounding.convert(),
						frame_border_size: s.frame_border_size.convert(),
						item_spacing: s.item_spacing.convert(),
						item_inner_spacing: s.item_inner_spacing.convert(),
						cell_padding: s.cell_padding.convert(),
						touch_extra_padding: s.touch_extra_padding.convert(),
						indent_spacing: s.indent_spacing.convert(),
						scrollbar_size: s.scrollbar_size.convert(),
						scrollbar_rounding: s.scrollbar_rounding.convert(),
						grab_min_size: s.grab_min_size.convert(),
						grab_rounding: s.grab_rounding.convert(),
						log_slider_deadzone: s.log_slider_deadzone.convert(),
						tab_rounding: s.tab_rounding.convert(),
						tab_border_size: s.tab_border_size.convert(),
						button_text_align: s.button_text_align.convert(),
						selectable_text_align: s.selectable_text_align.convert(),
						display_safe_area_padding: s.display_safe_area_padding.convert(),
						colors: meta_base.style.colors.into_iter().map(|(v, k)| (v, k.convert())).collect(),
					}
				})
			},
			
			issues: vec![
				Issue::UiTheme("Dark".to_string()),
				Issue::UiResolution("High".to_string()),
				Issue::Collection("Interface".to_string()),
			],
			
			..Default::default()
		};
		
		for (option, paths) in &files {
			if let Some((main, sub)) = option {
				let Some(opt) = meta.options.0.iter().find(|v| if let meta::OptionType::Option(v) = v {v.name == *main} else {false}) else {
					println!("No option exists with name {main}");
					for (p, _) in paths {
						println!("\t - {p}");
					}
					
					continue;
				};
				
				if let meta::OptionType::Option(opt) = opt {
					if let meta::OptionSettings::SingleFiles(sub_opt) = &opt.settings {
						if !sub_opt.options.iter().any(|v| v.name == *sub) {
							println!("No sub option exists for {main} with name {sub}");
							for (p, _) in paths {
								println!("\t - {p}");
							}
						}
					}
				}
			}
		}
		
		std::fs::write(Path::new(&args[3]).with_extension("json"), serde_json::to_vec(&meta)?)?;
	}
	
	// color paths log
	// let mut f = BufWriter::new(File::create("color_paths.log")?);
	// for (color, paths) in color_paths {
	// 	writeln!(f, "\n{color}")?;
	// 	let mut paths = paths.into_iter().collect::<Vec<_>>();
	// 	paths.sort();
	// 	for p in paths {
	// 		writeln!(f, "\t{p}")?;
	// 	}
	// }
	
	Ok(())
}

struct SvgResult {
	path: String,
	option: Option<(String, String)>,
	layers: Vec<(Option<String>, String)>,
}

fn split_svgs(data: &str) -> Result<Vec<SvgResult>, Error> {
	// let svg = xml::EventWriter::new(Cursor::new(Vec::new()));
	let mut svgs: HashMap<String, HashMap<String, Vec<(String, xml::EventWriter<Cursor<Vec<u8>>>)>>> = HashMap::new();
	let mut layer = 0;
	let mut root_attributes = Vec::new();
	let mut path = String::new();
	let mut option = String::new();
	let mut color_option = String::new();
	let mut g1 = Vec::new();
	let mut g2 = Vec::new();
	
	let xml_reader = xml::EventReader::from_str(data);
	let mut xml = Vec::new();
	for e in xml_reader {
		if let Ok(e) = e {
			match &e {
				xml::reader::XmlEvent::StartElement{name, attributes, ..} => {
					if layer == 0 && name.local_name.as_str() == "svg" {
						root_attributes = attributes.to_owned();
					}
					
					xml.push(e)
				}
				
				xml::reader::XmlEvent::EndElement{..} => xml.push(e),
				xml::reader::XmlEvent::CData(_) => xml.push(e),
				xml::reader::XmlEvent::Characters(_) => xml.push(e),
				
				_ => {}
			}
		}
	}
	
	let mut force_add = false;
	let mut force_add_layer = 0;
	let mut is_use = None;
	let mut add = Vec::new();
	let mut use_adds = HashMap::new();
	for e in xml.iter() {
		match e {
			xml::reader::XmlEvent::StartElement{name, attributes, namespace} => {
				let label = attributes.iter().find(|&v| v.name.local_name == "label");
				let p = label.map_or(false, |v| v.value.starts_with("+"));
				if !force_add && ((layer == 1 && name.local_name.as_str() == "defs") || p)  {
					force_add = true;
					force_add_layer = layer;
					is_use = if p {Some(attributes.iter().find(|&v| v.name.local_name == "id").unwrap().to_owned().value)} else {None};
				}
				
				if let Some(use_id) = &is_use {
					let add = use_adds.entry(use_id.clone()).or_insert_with(|| Vec::new());
					add.push(xml::writer::XmlEvent::StartElement {
						name: name.borrow(),
						namespace: namespace.borrow(),
						attributes: if layer == force_add_layer {
							attributes.iter().map(|v| v.borrow()).filter(|v| v.name.local_name != "style").collect()
						} else {
							attributes.iter().map(|v| v.borrow()).collect()
						},
					})
				} else if force_add {
					add.push(xml::writer::XmlEvent::StartElement {
						name: name.borrow(),
						namespace: namespace.borrow(),
						attributes: attributes.iter().map(|v| v.borrow()).collect(),
					})
				}
				
				layer += 1;
			}
			
			xml::reader::XmlEvent::EndElement{name} => {
				if let Some(use_id) = &is_use {
					let add = use_adds.entry(use_id.clone()).or_insert_with(|| Vec::new());
					add.push(xml::writer::XmlEvent::EndElement {
						name: Some(name.borrow()),
					})
				} else if force_add {
					add.push(xml::writer::XmlEvent::EndElement {
						name: Some(name.borrow()),
					})
				}
				
				layer -= 1;
				
				if layer == force_add_layer {
					force_add = false;
					is_use = None;
				}
			}
			
			xml::reader::XmlEvent::Characters(v) => {
				if let Some(use_id) = &is_use {
					let add = use_adds.entry(use_id.clone()).or_insert_with(|| Vec::new());
					add.push(xml::writer::XmlEvent::Characters(v));
				}
			}
			
			_ => {}
		}
	}
	
	let mut bad_branch = false;
	let mut bad_branch_layer = 0;
	let mut use_is_solved = false;
	for e in xml.iter() {
		match e {
			xml::reader::XmlEvent::StartElement{name, attributes, namespace} => {
				let label = attributes.iter().find(|&v| v.name.local_name == "label");
				if !bad_branch && ((layer == 1 && name.local_name.as_str() != "g") || label.map_or(false, |v| v.value.starts_with("_") || v.value.starts_with("+"))) {
					bad_branch = true;
					bad_branch_layer = layer;
				}
				
				if !bad_branch {
					match (layer, name.local_name.as_str()) {
						(_, "use") => {
							let href = &attributes.iter().find(|&v| v.name.local_name == "href").unwrap().value[1..];
							if let Some(add) = use_adds.get(href) {
								let layers = svgs.get_mut(&path).unwrap().get_mut(&option).unwrap();
								let len = layers.len();
								let layer = &mut layers[len - 1].1;
								
								layer.write(xml::writer::XmlEvent::StartElement {
									name: "g".into(),
									namespace: namespace.borrow(),
									attributes: attributes.iter().filter(|v| v.name.local_name != "href").map(|v| v.borrow()).collect()
								})?;
								
								for e in add {
									layer.write(e.clone())?;
								}
								
								layer.write(xml::writer::XmlEvent::EndElement {
									name: Some("g".into()),
								})?;
								
								use_is_solved = true;
							} else {
								let layers = svgs.get_mut(&path).unwrap().get_mut(&option).unwrap();
								let len = layers.len();
								layers[len - 1].1.write(xml::writer::XmlEvent::StartElement {
									name: name.borrow(),
									namespace: namespace.borrow(),
									attributes: attributes.iter().map(|v| v.borrow()).collect(),
								})?;
								
								use_is_solved = false;
							}
						}
						
						(1, "g") => {
							path = label.ok_or("Svg does not contain path label")?.value.trim().to_ascii_lowercase().to_owned();
							if path.contains("./") || path.contains(".\\") {
								panic!("{path} is invalid");
							}
							
							g1 = attributes.iter().filter(|v| v.name.local_name != "style").collect();
						}
						
						(2, "g") => {
							option = label.ok_or("Svg does not contain option label")?.value.trim().to_owned();
							g2 = attributes.iter().filter(|v| v.name.local_name != "style").collect();
						}
						
						(3, "g") => {
							color_option = label.ok_or("Svg does not contain color option label")?.value.trim().to_owned();
							
							let layers = svgs.entry(path.clone()).or_insert_with(|| HashMap::new())
								.entry(option.clone()).or_insert_with(|| Vec::new());
							
							let len = layers.len();
							if len == 0 || layers[len - 1].0 != color_option {
								let mut writer = xml::EventWriter::new(Cursor::new(Vec::new()));
								writer.write(xml::writer::XmlEvent::StartElement {
									name: "svg".into(),
									namespace: namespace.borrow(),
									attributes: root_attributes.iter().map(|v| v.borrow()).collect(),
								})?;
								
								for event in &add {
									writer.write(event.clone())?;
								}
								
								writer.write(xml::writer::XmlEvent::StartElement {
									name: "g".into(),
									namespace: namespace.borrow(),
									attributes: g1.iter().map(|v| v.borrow()).collect(),
								})?;
								
								writer.write(xml::writer::XmlEvent::StartElement {
									name: "g".into(),
									namespace: namespace.borrow(),
									attributes: g2.iter().map(|v| v.borrow()).collect(),
								})?;
								
								writer.write(xml::writer::XmlEvent::StartElement {
									name: "g".into(),
									namespace: namespace.borrow(),
									attributes: attributes.iter().filter(|v| v.name.local_name != "style").map(|v| v.borrow()).collect(),
								})?;
								
								layers.push((color_option.clone(), writer));
							}
						}
						
						(4.., _) => {
							let layers = svgs.get_mut(&path).unwrap().get_mut(&option).unwrap();
							let len = layers.len();
							layers[len - 1].1.write(xml::writer::XmlEvent::StartElement {
								name: name.borrow(),
								namespace: namespace.borrow(),
								attributes: patch_attributes(name, attributes).iter().map(|v| v.borrow()).collect(),
							})?;
						}
						
						_ => {}
					}
				}
				
				layer += 1;
			}
			
			xml::reader::XmlEvent::EndElement{name} => {
				layer -= 1;
				
				if !bad_branch {
					match (layer, name.local_name.as_str()) {
						(_, "use") => {
							if !use_is_solved {
								let layers = svgs.get_mut(&path).unwrap().get_mut(&option).unwrap();
								let len = layers.len();
								let layer = &mut layers[len - 1].1;
								layer.write(xml::writer::XmlEvent::EndElement {
									name: Some(name.borrow()),
								})?;
							}
						}
						
						(1, "g") => {
							path.clear();
						}
						
						(2, "g") => {
							option.clear();
						}
						
						(3, "g") => {
							color_option.clear();
						}
						
						(4.., _) => {
							let layers = svgs.get_mut(&path).unwrap().get_mut(&option).unwrap();
							let len = layers.len();
							let layer = &mut layers[len - 1].1;
							layer.write(xml::writer::XmlEvent::EndElement {
								name: Some(name.borrow()),
							})?;
						}
						
						_ => {}
					}
				}
				
				if layer == bad_branch_layer {
					bad_branch = false;
				}
			}
			
			xml::reader::XmlEvent::Characters(v) => {
				if layer >= 4 {
					if let Some(a) = svgs.get_mut(&path) {
						if let Some(layers) = a.get_mut(&option) {
							let len = layers.len();
							layers[len - 1].1.write(xml::writer::XmlEvent::Characters(v))?;
						}
					}
				}
			}
			
			_ => {}
		}
	}
	
	Ok(svgs.into_iter().flat_map(|(path, options)| {
		options.into_iter().flat_map(move |(option_unsplit, layers)| {
			let layers = layers.into_iter().map(|(color_option, mut svg)| {
				svg.write(xml::writer::XmlEvent::EndElement{name: Some("g".into())}).unwrap();
				svg.write(xml::writer::XmlEvent::EndElement{name: Some("g".into())}).unwrap();
				svg.write(xml::writer::XmlEvent::EndElement{name: Some("g".into())}).unwrap();
				svg.write(xml::writer::XmlEvent::EndElement{name: Some("svg".into())}).unwrap();
				
				(if color_option.trim().len() == 0 {
					None
				} else {
					Some(color_option)
				}, String::from_utf8(svg.into_inner().into_inner()).unwrap())
			}).collect::<Vec<(Option<String>, String)>>();
			
			option_unsplit.split(";").into_iter().map(|option|
				SvgResult {
					path: path.clone(),
					option: if option.trim().len() == 0 {
						None
					} else {
						let mut s = option.split(":");
						println!("{}", path);
						Some((s.next().unwrap().to_owned(), s.next().unwrap().to_owned()))
					},
					layers: layers.clone(),
				}
			).collect::<Vec<SvgResult>>()
		})
	}).collect())
}

fn render_svg(svg: SvgResult, target_root: &Path, font: &resvg::usvg::fontdb::Database/*, color_paths: &mut HashMap<String, HashSet<String>>*/) -> Result<(), Error> {
	let local_dir = if let Some((o1, o2)) = &svg.option {
		format!("{}/{o1}/{o2}", svg.path.clone())
	} else {
		svg.path.clone()
	};
	
	let dir = target_root.join(&local_dir);
	_ = std::fs::create_dir_all(&dir);
	
	if svg.layers.len() > 1 || svg.layers[0].0 != None { // composite info
		use aetherment::modman::{Path, composite::tex::*};
		
		let mut layers = Vec::new();
		for (i, (color_option, _layer)) in svg.layers.iter().enumerate().rev() {
			layers.push(Layer {
				name: format!("Layer{i}"),
				path: Path::Mod(format!("{local_dir}/{i}.tex")),
				blend: Blend::Normal,
				modifiers: if let Some(color_option) = color_option {
					// color_paths.entry(color_option.to_owned()).or_insert_with(|| HashSet::new()).insert(svg.path.clone());
					
					vec![
						Modifier::Color {
							value: OptionOrStatic::Option(ColorOption(color_option.to_owned()))
						}
					]
				} else {
					Vec::new()
				}
			});
		}
		
		std::fs::write(dir.join("comp.tex.comp"), serde_json::to_string(&Tex{layers: layers})?)?;
	}
	
	let opt = resvg::usvg::Options {
		// shape_rendering: resvg::usvg::ShapeRendering::OptimizeSpeed,
		// shape_rendering: resvg::usvg::ShapeRendering::CrispEdges,
		..Default::default()
	};
	
	for (i, (_color_option, layer)) in svg.layers.into_iter().enumerate() {
		let tree = resvg::usvg::Tree::from_str(&layer, &opt, &font)?;
		let size = tree.size().to_int_size();
		let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width(), size.height()).ok_or("Failed creating pixmap with specified size")?;
		resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
		
		{ // svg (debug purposes)
			std::fs::write(dir.join(format!("{i}.svg")), &layer)?
		}
		
		// { // png
		// 	pixmap.save_png(dir.join(format!("{i}.png")))?
		// }
		
		{ // tex
			for pixel in pixmap.pixels_mut() {
				let c = pixel.demultiply();
				// fuck you tiny_skia for making PremultipliedColorU8::from_rgba_unchecked private,
				// could've make a new vec but dont want to alloc memory
				*pixel = unsafe{std::mem::transmute::<_, _>([c.red(), c.green(), c.blue(), c.alpha()])};
			}
			
			save_tex(size.width() as u16, size.height() as u16, pixmap.data(), &dir.join(format!("{i}.tex")))?;
		}
	}
	
	Ok(())
}

// svg is kinda a mess, here we patch stuff so that we may have incorrect behaviour but it atleast shows up like it does in inkscape
fn patch_attributes<'a>(name: &xml::name::OwnedName, att: &Vec<xml::attribute::OwnedAttribute>) -> Vec<xml::attribute::OwnedAttribute> {
	let mut att = att.to_owned();
	
	// rect zero rx/ry resolving
	// https://razrfalcon.github.io/resvg-test-suite/svg-support-table.html#:~:text=Zero%20%60rx%60%20attribute%20resolving
	if name.local_name == "rect" {
		att.retain(|v| !(v.name.local_name == "rx" && v.value == "0"));
		att.retain(|v| !(v.name.local_name == "ry" && v.value == "0"));
	}
	
	att
}

pub fn save_tex(width: u16, height: u16, data: &[u8], path: &Path) -> Result<(), Error> {
	let mut f = BufWriter::new(File::create(path)?);
	
	// header
	f.write(&0x00800000u32.to_le_bytes())?;
	f.write(&0x1450u32.to_le_bytes())?;
	f.write(&width.to_le_bytes())?;
	f.write(&height.to_le_bytes())?;
	f.write(&0u16.to_le_bytes())?;
	f.write(&1u16.to_le_bytes())?;
	for v in [0u32, 1, 2] {
		f.write(&v.to_le_bytes())?;
	}
	for v in [80u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
		f.write(&v.to_le_bytes())?;
	}
	
	// body
	for v in data.chunks_exact(4) {
		f.write(&v[2].to_le_bytes())?;
		f.write(&v[1].to_le_bytes())?;
		f.write(&v[0].to_le_bytes())?;
		f.write(&v[3].to_le_bytes())?;
	}
	
	// png for debugging
	let img = image::RgbaImage::from_vec(width as _, height as _, data.to_vec()).unwrap();
	img.save(path.with_extension("png"))?;
	// match image::RgbaImage::from_vec(width as _, height as _, data.to_vec()) {
	// 	Some(img) => img.save(path.with_extension("png"))?,
	// 	None => println!("failed png {path:?}"),
	// }
	
	Ok(())
}