////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
//          FUNCIONTIONS IMPLEMENTATED BY AI              //
////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use plotters::prelude::*;

use crate::model::GraphAdj;

pub fn export_dot(graph: &GraphAdj, path: &[usize], path_out: &str) {
    let path_edges: HashSet<(usize, usize)> = path
        .windows(2)
        .map(|w| (w[0], w[1]))
        .collect();

    let mut f = File::create(path_out).unwrap();
    writeln!(f, "digraph G {{").unwrap();

    for (from, neighbors) in graph.data.iter().enumerate() {
        for &to in neighbors {
            let style = if path_edges.contains(&(from, to)) {
                r#"[color="red", penwidth=3]"#
            } else {
                ""
            };
            writeln!(f, "  {} -> {} {};", from, to, style).unwrap();
        }
    }

    if let (Some(&s), Some(&e)) = (path.first(), path.last()) {
        writeln!(f, "  {} [style=filled, fillcolor=lightgreen];", s).unwrap();
        writeln!(f, "  {} [style=filled, fillcolor=lightblue];", e).unwrap();
    }

    writeln!(f, "}}").unwrap();
}

/// Loads DIMACS `.co` coordinates. Nodes are 1-indexed in the file; returned
/// vector is 0-indexed. Coordinates are stored as f64 (lon, lat) scaled values.
pub fn load_coordinates(path: &str) -> Vec<(f64, f64)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut coords: Vec<(f64, f64)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with('p') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // p aux sp co <nodes>
            let nodes: usize = parts[4].parse().unwrap();
            coords.resize(nodes, (0.0, 0.0));
            continue;
        }

        if !line.starts_with('v') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let node: usize = parts[1].parse().unwrap();
        let x: f64 = parts[2].parse().unwrap();
        let y: f64 = parts[3].parse().unwrap();

        coords[node - 1] = (x, y);
    }

    coords
}

/// Renders the shortest path (and nearby background roads) to a PNG using
/// geographic coordinates from the `.co` file.
pub fn export_path_png(
    graph: &GraphAdj,
    coords: &[(f64, f64)],
    path: &[usize],
    path_out: &str,
) {
    assert!(!path.is_empty(), "path must not be empty");

    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    for &node in path {
        let (x, y) = coords[node];
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    let pad_x = (x_max - x_min).max(1.0) * 0.08;
    let pad_y = (y_max - y_min).max(1.0) * 0.08;
    x_min -= pad_x;
    x_max += pad_x;
    y_min -= pad_y;
    y_max += pad_y;

    let root = BitMapBackend::new(path_out, (7680, 4320)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    chart.configure_mesh().disable_mesh().draw().unwrap();

    // Background: edges whose endpoints fall inside the path viewport.
    let in_view = |n: usize| {
        let (x, y) = coords[n];
        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    };

    for (from, neighbors) in graph.data.iter().enumerate() {
        if !in_view(from) {
            continue;
        }
        for &to in neighbors {
            if !in_view(to) {
                continue;
            }
            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![coords[from], coords[to]],
                    RGBColor(210, 210, 210).stroke_width(1),
                )))
                .unwrap();
        }
    }

    // Shortest path in red.
    let path_points: Vec<(f64, f64)> = path.iter().map(|&n| coords[n]).collect();
    chart
        .draw_series(std::iter::once(PathElement::new(
            path_points,
            RED.stroke_width(3),
        )))
        .unwrap();

    // Start / end markers.
    let start = coords[path[0]];
    let end = coords[*path.last().unwrap()];

    chart
        .draw_series(PointSeries::of_element(
            vec![start],
            6,
            &RGBColor(40, 180, 70),
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))
        .unwrap();

    chart
        .draw_series(PointSeries::of_element(
            vec![end],
            6,
            &RGBColor(40, 100, 220),
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))
        .unwrap();

    root.present().unwrap();
}

/// Exports the shortest path as GeoJSON for browser viewers (e.g. geojson.io).
/// DIMACS `.co` values are millionths of a degree; this converts them to lon/lat.
pub fn export_path_geojson(coords: &[(f64, f64)], path: &[usize], path_out: &str) {
    assert!(!path.is_empty(), "path must not be empty");

    let mut f = File::create(path_out).unwrap();

    writeln!(f, "{{").unwrap();
    writeln!(f, r#"  "type": "FeatureCollection","#).unwrap();
    writeln!(f, r#"  "features": ["#).unwrap();

    // Path as a LineString.
    writeln!(f, "    {{").unwrap();
    writeln!(f, r#"      "type": "Feature","#).unwrap();
    writeln!(
        f,
        r#"      "properties": {{ "name": "shortest_path", "nodes": {} }},"#,
        path.len()
    )
    .unwrap();
    writeln!(f, r#"      "geometry": {{"#).unwrap();
    writeln!(f, r#"        "type": "LineString","#).unwrap();
    writeln!(f, r#"        "coordinates": ["#).unwrap();

    for (i, &node) in path.iter().enumerate() {
        let (x, y) = coords[node];
        let lon = x / 1_000_000.0;
        let lat = y / 1_000_000.0;
        let comma = if i + 1 == path.len() { "" } else { "," };
        writeln!(f, "          [{lon}, {lat}]{comma}").unwrap();
    }

    writeln!(f, "        ]").unwrap();
    writeln!(f, "      }}").unwrap();
    writeln!(f, "    }},").unwrap();

    // Start point.
    let (sx, sy) = coords[path[0]];
    writeln!(f, "    {{").unwrap();
    writeln!(f, r#"      "type": "Feature","#).unwrap();
    writeln!(
        f,
        r#"      "properties": {{ "name": "start", "node": {} }},"#,
        path[0]
    )
    .unwrap();
    writeln!(f, r#"      "geometry": {{"#).unwrap();
    writeln!(f, r#"        "type": "Point","#).unwrap();
    writeln!(
        f,
        r#"        "coordinates": [{}, {}]"#,
        sx / 1_000_000.0,
        sy / 1_000_000.0
    )
    .unwrap();
    writeln!(f, "      }}").unwrap();
    writeln!(f, "    }},").unwrap();

    // End point.
    let end_node = *path.last().unwrap();
    let (ex, ey) = coords[end_node];
    writeln!(f, "    {{").unwrap();
    writeln!(f, r#"      "type": "Feature","#).unwrap();
    writeln!(
        f,
        r#"      "properties": {{ "name": "end", "node": {} }},"#,
        end_node
    )
    .unwrap();
    writeln!(f, r#"      "geometry": {{"#).unwrap();
    writeln!(f, r#"        "type": "Point","#).unwrap();
    writeln!(
        f,
        r#"        "coordinates": [{}, {}]"#,
        ex / 1_000_000.0,
        ey / 1_000_000.0
    )
    .unwrap();
    writeln!(f, "      }}").unwrap();
    writeln!(f, "    }}").unwrap();

    writeln!(f, "  ]").unwrap();
    writeln!(f, "}}").unwrap();
}
