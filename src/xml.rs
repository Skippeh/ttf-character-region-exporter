use serde::Serialize;
use std::{fs::File, io::Write, path::Path};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CharacterRange {
    start: Start,
    end: End,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Start {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct End {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize)]
struct CharacterRegions {
    #[serde(rename = "CharacterRegion")]
    character_ranges: Vec<CharacterRange>,
}

impl CharacterRegions {
    fn new(ranges: &[crate::CharacterRange]) -> Self {
        let mut new_ranges = Vec::<CharacterRange>::new();

        for range in ranges {
            new_ranges.push(CharacterRange {
                start: Start {
                    value: range.start.to_string(),
                },
                end: End {
                    value: range.end.to_string(),
                },
            })
        }

        Self {
            character_ranges: new_ranges,
        }
    }
}

pub fn export_ranges(
    file_path: &Path,
    ranges: &[crate::CharacterRange],
) -> Result<(), anyhow::Error> {
    let document = CharacterRegions::new(ranges);
    let xml = quick_xml::se::to_string(&document)?;

    let mut file = File::create(file_path)?;
    file.write_all(xml.as_bytes())?;

    Ok(())
}
