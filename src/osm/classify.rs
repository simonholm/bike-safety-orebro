use osmpbfreader::OsmObj;
use geojson::FeatureCollection;

pub fn classify_missing_cycleways(_ways: &[OsmObj]) -> FeatureCollection {
    FeatureCollection { features: Vec::new(), bbox: None, foreign_members: None }
}

pub fn classify_intersections(_nodes: &[OsmObj]) -> FeatureCollection {
    FeatureCollection { features: Vec::new(), bbox: None, foreign_members: None }
}
