# Bike Safety Örebro

This project is an **early-stage, work-in-progress (WIP)** Rust pipeline for analyzing
bicycle infrastructure and potential danger zones in Örebro, Sweden.
It extracts OpenStreetMap (OSM) data, identifies missing cycleways, detects risky
intersections, and exports results as GeoJSON for visualization in GIS tools such as QGIS.

> **Status:** Very early scaffold.
> Most analysis logic is not implemented yet.
> The current version loads OSM data and exports empty GeoJSON layers.

---

## Goals

- Load and parse OSM PBF extracts (Örebro or full Sweden)
- Identify road segments missing cycleway infrastructure
- Detect complex or high-risk intersections
- Export GIS-ready layers for validation and visualization
- Support a hybrid Rust + QGIS workflow

---

## Getting Started

### 1. Download OSM Data

Get the Sweden extract from Geofabrik:
https://download.geofabrik.de/europe/sweden.html

Place the file at:

data/orebro.osm.pbf

### 2. Run the Analyzer

    cargo run

Outputs (currently empty):

results/danger_segments.geojson
results/danger_intersections.geojson

### 3. Visualize in QGIS

Open the generated GeoJSON files and apply styling from the qgis/ folder.

---

## Project Structure

data/         Input OSM PBF
results/      Output GeoJSON layers
src/osm/      Rust modules (load, classify, export)
qgis/         Optional QGIS styling
Cargo.toml    Dependencies

---

## Roadmap (WIP)

- Real classification of missing cycleways
- Intersection-degree / topology-based risk scoring
- Gap detection between cycleway segments
- CLI parameters (--city, --input, --output)
- QGIS project template for consistent styling

---

## License

Open-source. Contributions welcome once the core pipeline stabilizes.
