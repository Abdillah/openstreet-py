/** openstreet */
use std::fs;
use std::env;
use std::path::Path;

use openstreet::network::StreetNetwork;
use openstreet::map::Map;

// NOTE: For now this can't be used because we prefer the pickle version.
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
    let bytes = serde_json::to_string(&gra).unwrap();
    fs::write(path, bytes);

    // let gra5: StreetNetwork = serde_json::from_str(std::str::from_utf8(&bytes2).unwrap()).unwrap();


    // let bytes = gra4.serialize();
    // fs::write(path, bytes);

    let bytes2 = fs::read(path).expect("File read failed");
    // let mut gra2: StreetNetwork = serde_json::from_str(std::str::from_utf8(&bytes2).unwrap()).unwrap();
    let mut gra2 = StreetNetwork::deserialize(bytes2);
    println!("{:?}", gra2.shortest_path(4137262376, 4137262384));
}
