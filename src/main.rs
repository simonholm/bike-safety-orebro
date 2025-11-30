mod osm;

use anyhow::Result;
use osm::load::load_osm_extract;
use osm::classify::{classify_missing_cycleways, classify_intersections};
use osm::export::export_geojson;

fn main() -> Result<()> {
    env_logger::init();

    let path = "data/orebro.osm.pbf";
    println!("Loading OSM extract: {}", path);

    let (nodes, ways) = load_osm_extract(path)?;

    let danger_segments = classify_missing_cycleways(&ways);
    let danger_nodes = classify_intersections(&nodes);

    export_geojson(&danger_segments, "results/danger_segments.geojson")?;
    export_geojson(&danger_nodes, "results/danger_intersections.geojson")?;

    println!("Done.");
    Ok(())
}
