/** openstreet */
use std::fs;
use std::env;
use std::path::Path;

use openstreet::network::StreetNetwork;
use openstreet::map::Map;

fn main() {
    // Prints each argument on a separate line
    let mapfilepath = match env::args().nth(1) {
        Some(p) => p,
        None => panic!("Filepath not given, please add OSM map file as argument"),
    };

    let outpath = match env::args().nth(2) {
        Some(p) => p,
        None => panic!("Filepath not given, please add OSM map file as argument"),
    };


    println!("Creating Map!");
    let map = Map::new(mapfilepath);

    println!("Into StreetNetwork!");
    let gra = StreetNetwork::new(&map, vec![
        "primary"      , "secondary"      , "tertiary",
        "primary_link" , "secondary_link" , "tertiary_link",
        "residential"  , "service"
    ]);

    let path: &Path = Path::new(&outpath);
    gra.serialize(path);
}
