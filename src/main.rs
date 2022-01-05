use std::path::Path;

use anyhow::Context;
use serde::Serialize;
use ttf_parser::PlatformId;

mod xml;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterRange {
    start: u32,
    end: u32,
}

fn main() -> anyhow::Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: font-file.ttf");
        return Ok(());
    }

    let font_data = match std::fs::read(&args[1]) {
        Ok(f) => f,
        Err(e) => anyhow::bail!(e),
    };

    let face = match ttf_parser::Face::from_slice(&font_data, 0) {
        Ok(f) => f,
        Err(e) => anyhow::bail!(e),
    };

    println!("Glyphs in font: {}", face.number_of_glyphs());

    let cmap = face.tables().cmap.context("cmap table not found")?;
    let mut codepoints = Vec::<u32>::new();

    for table in cmap.subtables {
        if table.platform_id == PlatformId::Windows {
            table.codepoints(|id| {
                codepoints.push(id);
            });
        }
    }

    if codepoints.is_empty() {
        anyhow::bail!("No characters found");
    }

    codepoints.sort_unstable();

    println!("Found {} codepoints", codepoints.len());

    let mut start: u32 = 0;
    let mut last_cp: Option<u32> = None;
    let mut ranges = Vec::<CharacterRange>::new();
    let codepoints_len = codepoints.len();

    for cp in codepoints {
        // Check that the codepoint is valid and is not a surrogate value
        if cp > 0x10ffff || (0x00d800..=0x00dfff).contains(&cp) {
            continue;
        }

        match last_cp {
            Some(last_cp_val) => {
                if cp > last_cp_val + 1 || (cp as usize) == codepoints_len {
                    // Check if last_cp and cp aren't equal (they would be if last cp is at the end of the array)
                    if last_cp_val != cp {
                        ranges.push(CharacterRange {
                            start,
                            end: last_cp_val,
                        });
                    }

                    last_cp = None;
                } else {
                    last_cp = Some(cp);
                }
            }
            None => {
                start = cp;
                last_cp = Some(cp);
            }
        }
    }

    println!("Found {} character ranges", ranges.len());

    for range in &ranges {
        println!("{} - {}", range.start, range.end);
    }

    xml::export_ranges(Path::new("output.xml"), &ranges).context("Exporting to xml failed")?;

    Ok(())
}
