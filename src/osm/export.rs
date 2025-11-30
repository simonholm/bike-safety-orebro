use anyhow::Result;
use geojson::FeatureCollection;
use std::fs::File;
use std::io::Write;

pub fn export_geojson(fc: &FeatureCollection, path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    let json = serde_json::to_string_pretty(fc)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
