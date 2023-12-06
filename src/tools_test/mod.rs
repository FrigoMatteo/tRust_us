use crate::{
    robotics_lib::interface::{craft, discover_tiles, look_at_sky, teleport, where_am_i},
    robotics_lib::world::tile::{Tile,TileType,Content},
    robotics_lib::world::*,

};
fn generate_map_go_interface() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile {
            tile_type: TileType::Street,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::ShallowWater,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: TileType::DeepWater,
            content: Content::None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::Sand,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: TileType::Hill,
            content: Content::None,
            elevation: 4,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: TileType::Lava,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::Snow,
            content: Content::None,
            elevation: 7,
        },
        Tile {
            tile_type: TileType::Mountain,
            content: Content::None,
            elevation: 9,
        },
    ]);
    map
}




#[test]
fn rr(){

}