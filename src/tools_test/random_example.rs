use std::collections::HashMap;
use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{debug, robot_view, Tools};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::*;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::World;
use robotics_lib::world::worldgenerator::Generator;
use crate::tools::gps::gps;
use crate::tools_test::my_position;

#[test]
fn random_example() {
    struct WorldGeneratorRandom {
        size: usize,
    }
    impl WorldGeneratorRandom {
        fn new(size: usize) -> Self {
            WorldGeneratorRandom { size }
        }
    }
    impl Generator for WorldGeneratorRandom {
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
            let mut rng = rand::thread_rng();
            let mut map: Vec<Vec<Tile>> = Vec::new();
            // Initialize the map with default tiles
            for _ in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for _ in 0..self.size {
                    let i_tiletype = rng.gen_range(0..=9);
                    let i_content = rng.gen_range(0..=2);
                    let elevation = rng.gen_range(0..=9);
                    /*let mut tile_type = match i_tiletype {
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
                    };*/
                    let tile_type = Grass;
                    let content = match i_content {
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
                    };
                    //let content=None;
                    row.push(Tile {
                        tile_type,
                        content,
                        elevation,
                    });
                }
                map.push(row);
            }
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

            let max_score = rand::random::<f32>();

            (map, (0, 0), environmental_conditions, max_score, Option::None)
        }
    }

    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            let (map, dimension, (x_robot, y_robot)) = debug(self, world);
            for i in &map {
                for j in i {
                    print!(" |{:?} c={}| ", j.tile_type, j.content);
                }
                println!();
            }
            println!("\n");
            for i in &map {
                for j in i {
                    print!(" |{}| ", j.elevation);
                }
                println!();
            }
            my_position(self, world);
            robot_view(self, world);
            let res = gps(self, (1, 1), world, Option::None);
            println!("{:?}", res);

        }
        fn handle_event(&mut self, event: Event) {
            println!("{:?}", event);
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
    //We create the robot
    let r = MyRobot(Robot::new());
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];
    //We create the generator
    let mut generator: WorldGeneratorRandom = WorldGeneratorRandom::new(4);

    //We create the runner(Where we call the generator.gen()
    //We create and position our robot
    //Return the instaziation of the robot
    let run = Runner::new(Box::new(r), &mut generator, tools);
    // We initialize the robot
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}