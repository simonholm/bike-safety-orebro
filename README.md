# Bike Safety Örebro

This project analyzes bicycle infrastructure and potential danger zones in Örebro
using a lightweight Rust-based pipeline. The goal is to extract OpenStreetMap data,
identify missing cycleways, detect risky intersections, and export results as
GeoJSON for visualization in QGIS or other GIS tools.

## What This Project Does
- Loads a local OSM PBF extract (Örebro or Sweden-wide)
- Classifies missing cycleways and risky intersections
- Exports results into the results/ directory as GeoJSON

## Getting Started
### 1. Download OSM Data
Get Sweden extract from Geofabrik and place it at data/orebro.osm.pbf

### 2. Run the Analyzer
    cargo run

### 3. Visualize in QGIS
Open the GeoJSON files in results/ and style them freely.

## Project Structure
data/     OSM input
results/  GeoJSON output
src/osm/  Rust modules
qgis/     (Optional) styling

## Status
Functional scaffold; analysis logic will expand over time.
