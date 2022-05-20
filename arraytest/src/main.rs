#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use derivative::Derivative;

struct GroundTileData {
    #[derivative(Default(value = "false"))]
    coal_ore: bool,
    #[derivative(Default(value = "false"))]
    iron_ore: bool,
    #[derivative(Default(value = "false"))]
    copper_ore: bool,
}
struct BuildingTileData {
    #[derivative(Default(value = "false"))]
    insertor: bool,
    #[derivative(Default(value = "false"))]
    transport_belt: bool,
}
struct TileData {
    ground_data: GroundTileData,
    building_data: BuildingTileData,
}
fn main() {
    let mut map_array_2d: [[TileData; 1024]; 1024];
    map_array_2d[0][0].ground_data.coal_ore = true;
    println!("{:?}", map_array_2d[0][0].ground_data.coal_ore)
}