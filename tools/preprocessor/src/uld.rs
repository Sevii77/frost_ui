use std::path::Path;
use aetherment::noumenon_::format::game::uld::*;

pub fn ulds(target_root: &Path) -> Result<Vec<String>, crate::Error> {
	let files_root = target_root.join("files");
	let n = aetherment::noumenon().ok_or("Invalid Noumenon")?;
	
	let uld = |name, do_font, color: Option<&[&str]>| -> Result<String, crate::Error> {
		let path = format!("ui/uld/{name}.uld");
		println!("{path}");
		
		let mut u = n.file::<Uld>(&path)?;
		if let Some(paths) = color {
			for a in &mut u.assets {
				// println!(" - {}", a.path);
				
				if paths.contains(&a.path.to_ascii_lowercase().as_str()) {
					a.path = format!("{}_frost.tex", &a.path[..a.path.len() - 4])
				}
			}
		}
		
		let do_nodes = |nodes: &mut [NodeData]| {
			for n in nodes {
				if let Node::Text(tn) = &mut n.node {
					// println!("clr: {}; edge: {}; font: {:?}; sheet: {:?}; unk: {}", tn.color, tn.edge_color, tn.font, tn.sheet_type, tn.unk2);
					
					if color.is_some() {
						tn.color = match tn.color {
							1 => 7001,
							2 => 7001,
							3 => 7001,
							4 => 7001,
							5 => 7001,
							6 => 7001,
							7 => 7001,
							// 2 => 7002,
							// 3 => 7003,
							// 4 => 7004,
							// 5 => 7005,
							// 6 => 7006,
							// 7 => 7007,
							50 => 7001,
							51 => 7001, // #000000 > #FFFFFF
							// 55 => 7002, // #333333 > #C8C8C8
							55 => 7001, // #333333 > #FFFFFF
							_ => tn.color,
						};
						
						tn.edge = false;
					}
					
					if do_font && tn.font == FontType::Jupiter {
						tn.font = FontType::TrumpGothic
					}
				}
			}
		};
		
		for c in &mut u.components {
			do_nodes(&mut c.nodes);
		}
		
		for w in &mut u.widgets {
			do_nodes(&mut w.nodes);
		}
		
		u.write(&mut std::io::BufWriter::new(std::fs::File::create(&files_root.join(&path))?))?;
		
		Ok(path)
	};
	
	Ok(vec![
		uld("journaldetail", false, Some(&["ui/uld/journal_detail.tex"]))?,
		uld("character", true, None)?,
		// uld("gcarmymemberprofile", false, None)?,
	])
}