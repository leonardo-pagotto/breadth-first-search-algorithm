use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use model::GraphAdj;
use algorithms::bfs;
use tools::{export_path_geojson, export_path_png, load_coordinates};

mod model;
mod algorithms;
mod tools;

fn main()
{
    println!("Loading graph...");
    let mut graph = GraphAdj::new();
    extract_dataset(&mut graph);

    println!("Loading coordinates...");
    let coords = load_coordinates("dataset/USA-road-d.USA.co");    

    loop {
        let mut start = String::new();
        let mut end = String::new();

        println!("Enter start node (0-{}):", coords.len() - 1);
        stdin().read_line(&mut start).unwrap();
        println!("Enter end node (0-{}):", coords.len() - 1);
        stdin().read_line(&mut end).unwrap();

        let start: usize = start.trim().parse().unwrap();
        let end: usize = end.trim().parse().unwrap();

        println!("Finding shortest path...");
        let shortest_path = bfs(&graph, start, end);

        let Some(ref path) = shortest_path else {
            println!("No path found between {} and {}", start, end);
            continue;
        };

        export_path_png(&graph, &coords, path, "path.png");
        println!("PNG written to path.png ({} nodes)", path.len());

        export_path_geojson(&coords, path, "path.geojson");
        println!("GeoJSON written to path.geojson — open at https://geojson.io and import the file to see the path on a map ({} nodes)", path.len());
    }
}


fn extract_dataset(graph: &mut GraphAdj) {
    let file = File::open("dataset/USA-road-d.USA.gr").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        let line = line.unwrap();

        if line.starts_with('p') { 
            let line_splited: Vec<&str> = line.split_whitespace().collect();
            let nodes: usize = line_splited[2].parse().unwrap();

            graph.setup(nodes);
            
            continue;
        }

        if !line.starts_with('a') { continue; }

        let line_splited: Vec<&str> = line.split_whitespace().collect();

        let from: usize = line_splited[1].parse().unwrap();
        let to: usize = line_splited[2].parse().unwrap();

        graph.add_node(from - 1, to - 1);
    }
}
