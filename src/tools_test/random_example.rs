use std::collections::HashMap;
use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{debug, Direction, one_direction_view, robot_map, robot_view};
use robotics_lib::interface::Direction::{Left, Right};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::*;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::World;
use robotics_lib::world::world_generator::Generator;
use crate::tools::actuator::actuator;
use crate::tools::gps::Goal::Coordinates;
use crate::tools::gps::gps;
use crate::tools::gps::Command::Control;


fn visualize(robot:&mut impl Runnable,world:&mut World){
    for _ in 0..2{
        let commands=&[Control(Right)];
        let _=actuator(commands,10,robot,world);
        let _=one_direction_view(robot,world,Direction::Down,4);
    }
    for _ in 0..2{
        let commands=&[Control(Left)];
        let _=actuator(commands,10,robot,world);
    }
    return;
    let map=robot_map(world).unwrap();
    println!("\nWhat i discovered:");
    for i in &map{
        for j in i{
            if j.is_none(){print!(" |None| ")}
            else{print!(" |{:?}| ",j.as_ref().unwrap().tile_type)}
        }
        println!();
    }

}
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
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..self.size{
                let tile_type=Grass;
                let content=None;
                row.push(Tile{
                    tile_type,content,elevation:0,
                })
            }
            map.push(row);
            for i in 1..self.size {
                let mut row: Vec<Tile> = Vec::new();
                if i == self.size - 1 {
                    for _ in 0..self.size-1 {
                        let i_tiletype = rng.gen_range(0..=9);
                        let i_content = rng.gen_range(0..=1);
                        let elevation = rng.gen_range(0..=9);
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
                        let mut content = None;
                        if tile_type != Lava && tile_type != DeepWater && tile_type.properties() != Teleport(false).properties() && tile_type!=ShallowWater{
                            content = match i_content {
                                1 => Rock(2),
                                | _ => None,
                            };
                        } else {
                            content = None;
                        }
                        row.push(Tile {
                            tile_type,
                            content,
                            elevation,
                        });
                    }
                    let tile_type=Grass;
                    let content=None;
                    let elevation=0;
                    row.push(Tile{
                        tile_type,
                        content,
                        elevation,
                    });
                    map.push(row);
                } else {
                    for _ in 0..self.size {
                        let i_tiletype = rng.gen_range(0..=9);
                        let i_content = rng.gen_range(0..=1);
                        let elevation = rng.gen_range(0..=9);
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
                        let mut content = None;
                        if tile_type != Lava && tile_type != DeepWater && tile_type.properties() != Teleport(false).properties() && tile_type!=ShallowWater && tile_type!=Street{
                            content = match i_content {
                                1 => Rock(2),
                                | _ => None,
                            };
                        } else {
                            content = None;
                        }
                        row.push(Tile {
                            tile_type,
                            content,
                            elevation,
                        });
                    }
                    map.push(row);
                }
            }
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

            let max_score = rand::random::<f32>();

            (map, (0, 0), environmental_conditions, max_score, Option::None)
        }
    }

    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            let map= debug(self, world);
            for i in &map.0 {
                for j in i {
                    print!(" |{:?} c={}| ", j.tile_type, j.content);
                }
                println!();
            }
            println!("\n");
            for i in &map.0 {
                for j in i {
                    print!(" |{}| ", j.elevation);
                }
                println!();
            }
            visualize(self,world);
            //my_position(self, world);
            robot_view(self, world);
            let res = gps(self, Coordinates(3, 3), world, Option::None);
            println!("Commands:{:?}", res);

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
    //We create the generator
    let mut generator: WorldGeneratorRandom = WorldGeneratorRandom::new(4);

    //We create the runner(Where we call the generator.gen()
    //We create and position our robot
    //Return the instaziation of the robot
    let run = Runner::new(Box::new(r), &mut generator);
    // We initialize the robot
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}