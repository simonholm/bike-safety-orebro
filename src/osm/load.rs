use anyhow::Result;
use osmpbfreader::{OsmObj, OsmPbfReader};
use std::fs::File;

pub fn load_osm_extract(path: &str) -> Result<(Vec<OsmObj>, Vec<OsmObj>)> {
    let file = File::open(path)?;
    let mut reader = OsmPbfReader::new(file);

    let mut nodes = Vec::new();
    let mut ways = Vec::new();

    for obj in reader.iter() {
        let obj = obj?;
        match obj {
            OsmObj::Node(_) => nodes.push(obj),
            OsmObj::Way(_) => ways.push(obj),
            _ => {}
        }
    }

    Ok((nodes, ways))
}
