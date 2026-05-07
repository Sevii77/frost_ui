use std::{collections::HashSet, path::Path};
use aetherment::noumenon::format::{external::Bytes, game::uld::*};

pub fn ulds(target_root: &Path) -> Result<Vec<String>, crate::Error> {
	let files_root = target_root.join("files");
	let n = aetherment::noumenon_instance().ok_or("Invalid Noumenon")?;
	
	let uld = |name, do_font, color: Option<(&[&str], &[(u32, u32)], bool)>, add_color: Option<&[([i16; 3], [i16; 3])]>| -> Result<String, crate::Error> {
		let path = format!("ui/uld/{name}.uld");
		println!("{path}");
		
		let mut u = n.file::<Uld>(&path)?;
		if let Some((paths, _, _)) = color {
			for a in &mut u.assets {
				// println!(" - {}", a.path);
				
				if paths.contains(&a.path.to_ascii_lowercase().as_str()) {
					a.path = format!("{}_frost.tex", &a.path[..a.path.len() - 4])
				}
			}
		}
		
		let mut done_timelines = HashSet::new();
		let mut do_nodes = |nodes: &mut [NodeData]| {
			for n in nodes {
				if let Some(colors) = add_color {
					for (org, new) in colors {
						if n.add_red == org[0] && n.add_green == org[1] && n.add_blue == org[2] {
							n.add_red = new[0];
							n.add_green = new[1];
							n.add_blue = new[2];
						}
					}
				}
				
				match &mut n.node {
					Node::Text(tn) => {
						if let Some((_, colors, _remove_timeline)) = color {
							println!("\t{}", tn.color);
							if let Some(c) = colors.iter().find(|v| v.0 == tn.color).map(|v| v.1) {
								tn.color = c;
								
								// if remove_timeline && n.timeline_id != 0 {
								// 	n.timeline_id = 0;
								// }
							}
							
							tn.edge = false;
							
							if !done_timelines.contains(&n.timeline_id) {
								if let Some(timeline) = u.timelines.iter_mut().find(|v| v.id == n.timeline_id as u32) {
									for frame in &mut timeline.frames1 {
										for keygroup in &mut frame.keygroups {
											match (keygroup.usage, &mut keygroup.frames) {
												(KeyUsage::TextColor, Keyframes::UShort1(frames)) => {
													for frame in frames {
														if let Some(c) = colors.iter().find(|v| v.0 == frame.value as u32).map(|v| v.1) {
															println!("\tchanging timeline text color");
															frame.value = c as u16;
														}
													}
												}
												
												(KeyUsage::TextColor, Keyframes::UInt1(frames)) => {
													for frame in frames {
														if (frame.value >> 24) & 1 != 1 {
															panic!("TexColor keyframe value does not have 1 in its 24th bit {}", frame.value);
														}
														
														if let Some(c) = colors.iter().find(|v| v.0 == frame.value & 0xFFFFFF).map(|v| v.1) {
															println!("\tchanging timeline text color");
															frame.value = c & 0xFFFFFF | 1 << 24;
														}
													}
												}
												
												_ => {}
											}
										}
									}
								}
								
								done_timelines.insert(n.timeline_id);
							}
						}
						
						if do_font && tn.font == FontType::Jupiter {
							tn.font = FontType::TrumpGothic
						}
					}
					
					Node::Image(_) => {
						if !done_timelines.contains(&n.timeline_id) {
							if let Some(timeline) = u.timelines.iter_mut().find(|v| v.id == n.timeline_id as u32) {
								for frame in &mut timeline.frames1 {
									for keygroup in &mut frame.keygroups {
										match (keygroup.usage, &mut keygroup.frames) {
											(KeyUsage::NodeColor, Keyframes::Color(frames)) => {
												for frame in frames {
													if let Some(colors) = add_color {
														for (org, new) in colors {
															if frame.add_red == org[0] && frame.add_green == org[1] && frame.add_blue == org[2] {
																println!("\tchanging timeline node color");
																frame.add_red = new[0];
																frame.add_green = new[1];
																frame.add_blue = new[2];
															}
														}
													}
												}
											}
											
											_ => {}
										}
									}
								}
							}
							
							done_timelines.insert(n.timeline_id);
						}
					}
					
					_ => {}
				}
			}
		};
		
		for c in &mut u.components {
			do_nodes(&mut c.nodes);
		}
		
		for w in &mut u.widgets {
			if color.is_some() {
				w.theme_support = true;
			}
			
			do_nodes(&mut w.nodes);
		}
		
		if color.is_some() {
			for a in &mut u.assets {
				a.theme_supported_bitmask = 0;
			}
		}
		
		u.write(&mut std::io::BufWriter::new(std::fs::File::create(&files_root.join(&path))?))?;
		
		Ok(path)
	};
	
	let foreground_turner = [
		(1, 1),
		(2, 1),
		(3, 1),
		(4, 1),
		(5, 1),
		(6, 1),
		(7, 1),
		(11, 1),
		(30, 2),
		(32, 32),
		(65, 65),
		(77, 2),
		(50, 1),
		(51, 1),
		(55, 1),
	];
	
	let brown_remover = [
		([50, 0, 0], [0, 0, 0]),
		([50, 25, 0], [0, 0, 0]),
		([100, 75, 0], [0, 0, 0]),
		([100, 75, 50], [0, 0, 0]),
		([116, 91, 16], [16, 16, 16]),
		([100, 91, 16], [16, 16, 16]),
		([-128, -220, -255], [0, 0, 0]),
	];
	
	Ok(vec![
		uld("journaldetail", false, Some((&["ui/uld/journal_detail.tex"], &foreground_turner, false)), None)?,
		
		uld("pvp", false, Some((&[], &foreground_turner, true)), Some(&brown_remover))?,
		uld("pvpcolosseum", false, Some((&[], &foreground_turner, true)), Some(&brown_remover))?,
		uld("pvpfrontline", false, Some((&[], &foreground_turner, true)), Some(&brown_remover))?,
		uld("pvpsimulation", false, Some((&[], &foreground_turner, true)), Some(&brown_remover))?,
		uld("pvpactionjob", false, Some((&[], &foreground_turner, true)), Some(&brown_remover))?,
		uld("pvpactionadditional", false, Some((&[], &foreground_turner, true)), Some(&brown_remover))?,
		uld("pvpactionquickchat", false, Some((&[], &foreground_turner, true)), Some(&brown_remover))?,
		uld("pvpcharacter", false, Some((&[], &foreground_turner, false)), Some(&brown_remover))?,
	])
}