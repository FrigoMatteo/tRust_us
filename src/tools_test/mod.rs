use std::collections::HashMap;
use crate::{
    robotics_lib::world::tile::{Tile,TileType,Content},
    robotics_lib::world::tile::TileType::*,
    robotics_lib::world::tile::Content::*,
    robotics_lib::world::environmental_conditions::EnvironmentalConditions,
    robotics_lib::world::environmental_conditions::WeatherType::*,
    robotics_lib::runner::Robot,
    robotics_lib::runner::Runnable,
    robotics_lib::runner::backpack::BackPack,
    robotics_lib::event::events::Event,
    robotics_lib::energy::Energy,
    robotics_lib::world::coordinates::Coordinate,
    robotics_lib::world::World,
    robotics_lib::utils::calculate_cost_go_with_environment,
    robotics_lib::interface::{craft, debug, destroy, go, look_at_sky, teleport, Direction::*},
};

use rand::Rng;
use robotics_lib::interface::{Tools, where_am_i};
use robotics_lib::runner::Runner;
use robotics_lib::world::worldgenerator::Generator;


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
fn generate_map_go_interface() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:1,
        },
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
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:2,
        },
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
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:1,
        },
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
    map.push(vec![
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:1,
        },
        Tile {
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::Sand,
            content: Content::Rock(20),
            elevation: 5,
        },
        Tile {
            tile_type: TileType::Mountain,
            content: Content::None,
            elevation: 4,
        },
    ]);
    for i in &map{
        for j in i{
            print!(" |{:?} C:{}| ",j.tile_type,j.content);
        }
        println!();
    }

    println!("\n\n");

    for i in &map{
        for j in i{
            print!(" |{}| ",j.elevation);
        }
        println!();
    }
    map
}

struct WorldGeneratorEample{
    size:usize,
}
impl robotics_lib::world::worldgenerator::Generator for WorldGeneratorEample{
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
        let mut rng = rand::thread_rng();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // Initialize the map with default tiles
        for _ in 0..self.size {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..self.size {
                let i_tiletype = rng.gen_range(0..=9);
                let i_content = rng.gen_range(0..=15);
                let tile_type = match i_tiletype {
                    | 0 => DeepWater,
                    | 1 => ShallowWater,
                    | 2 => Sand,
                    | 3 => Grass,
                    | 4 => Street,
                    | 5 => Hill,
                    | 6 => Mountain,
                    | 7 => Snow,
                    | 8 => Lava,
                    | 9 => Teleport(false),
                    | _ => Grass,
                };
                /*let content = match i_content {
                    | 0 => Rock(4),
                    | 1 => Tree(2),
                    | 2 => Garbage(2),
                    | 3 => Fire,
                    | 4 => Coin(2),
                    | 5 => Bin(2..3),
                    | 6 => Crate(2..3),
                    | 7 => Bank(3..54),
                    | 8 => Content::Water(20),
                    | 9 => Content::None,
                    | 10 => Fish(3),
                    | 11 => Market(20),
                    | 12 => Content::Building,
                    | 13 => Content::Bush(2),
                    | 14 => Content::JollyBlock(2),
                    | 15 => Content::Scarecrow,
                    | _ => Content::None,
                };*/
                let content=None;
                row.push(Tile {
                    tile_type,
                    content,
                    elevation: 0,
                });
            }
            map.push(row);
        }
        let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

        let max_score = rand::random::<f32>();

        (map, (0, 0), environmental_conditions, max_score,Option::None)
    }
}

struct MyRobot(Robot);
impl Runnable for MyRobot{
    fn process_tick(&mut self, world: &mut World) {
        for _ in 0..1 {
            let environmental_conditions = look_at_sky(world);
            println!(
                "Daytime: {:?}, Time:{:?}, Weather: {:?}\n",
                environmental_conditions.get_time_of_day(),
                environmental_conditions.get_time_of_day_string(),
                environmental_conditions.get_weather_condition()
            );
        }
    }
    fn handle_event(&mut self, event: Event) {
        println!();
        println!("{:?}", event);
        println!();
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}


#[test]
fn rr(){
    //let map=generate_map_go_interface();
    let mut wg=WorldGeneratorEample{
        size:6,
    };
    println!("\n");
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];

    let my_robot=MyRobot(Robot::new());
    let init_run=Runner::new(Box::new(my_robot),&mut wg,tools);
    let mut runner;
    if init_run.is_ok(){
        runner=init_run.unwrap();
    }else{
        return;
    }
    let _=runner.game_tick();
    //let here=where_am_i(&runner,);
}