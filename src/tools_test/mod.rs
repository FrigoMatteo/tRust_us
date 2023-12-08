mod show_example1;
mod generated_example;
mod random_example;
use crate::{
    robotics_lib::world::tile::Tile,
    robotics_lib::world::tile::TileType::*,
    robotics_lib::world::tile::Content::*,
    robotics_lib::runner::Runnable,
    robotics_lib::world::World,
    robotics_lib::interface::where_am_i,
};


//Helper functions:
pub (crate) fn my_position(robot:&impl Runnable,world:&World ){
    let (view,(x,y))=where_am_i(robot,world);
    println!();
    println!("I am at x:{} and y:{}",x,y);
    println!("Thats what i see:");
    for i in &view{
        for j in i{
            if j.is_some(){
                print!("| {:?} c:{} |",j.to_owned().unwrap().tile_type,j.to_owned().unwrap().content);
            }else{
                print!("| None |");
            }
        }
        println!();
    }
    println!("I have this quantity of energy:{}\n",robot.get_energy().get_energy_level());
}

/*
*  MAP:
*    ______________________________________
*   |            |            |            |
*   |   Street   | Shallow W. |  DeepWater |
*   |    3 el    |   2 el     |    1 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Grass   |    Sand    |    Hill    |
*   |    3 el    |    2 el    |    4 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |   Lava     |    Snow    |  Mountain  |
*   |   3 el     |    7 el    |    9 el    |
*   |____________|____________|____________|
*
*
*/
pub (crate) fn generate_map() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile{
            tile_type:Grass,
            content:None,
            elevation:1,
        },
        Tile {
            tile_type:Street,
            content: None,
            elevation: 3,
        },
        Tile {
            tile_type: ShallowWater,
            content: None,
            elevation: 2,
        },
        Tile {
            tile_type: DeepWater,
            content: None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:Grass,
            content:None,
            elevation:2,
        },
        Tile {
            tile_type: Grass,
            content: None,
            elevation: 3,
        },
        Tile {
            tile_type: Sand,
            content: None,
            elevation: 2,
        },
        Tile {
            tile_type: Hill,
            content: None,
            elevation: 4,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:Grass,
            content:None,
            elevation:1,
        },
        Tile {
            tile_type: Lava,
            content: None,
            elevation: 3,
        },
        Tile {
            tile_type: Snow,
            content: None,
            elevation: 7,
        },
        Tile {
            tile_type: Mountain,
            content: None,
            elevation: 9,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:Grass,
            content:None,
            elevation:1,
        },
        Tile {
            tile_type: Grass,
            content:None,
            elevation: 3,
        },
        Tile {
            tile_type: Sand,
            content: Rock(20),
            elevation: 5,
        },
        Tile {
            tile_type: Mountain,
            content: None,
            elevation: 4,
        },
    ]);
    map
}
