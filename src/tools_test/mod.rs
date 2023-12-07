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
    robotics_lib::runner::Runner,
    robotics_lib::event::events::Event,
    robotics_lib::energy::Energy,
    robotics_lib::world::coordinates::Coordinate,
    robotics_lib::world::worldgenerator::Generator,
    robotics_lib::world::World,
    robotics_lib::utils::{go_allowed, LibError, calculate_cost_go_with_environment},
    robotics_lib::interface::{Direction,Tools,where_am_i,craft, debug, destroy, go, look_at_sky, teleport, Direction::*},
};

use rand::Rng;
use robotics_lib::utils::LibError::NotEnoughEnergy;

//Attutatore modified
pub fn attuatore (comandi: &[Direction], costo:usize, robot: &mut impl Runnable,world: &mut World) -> Result<(), LibError>{
    return match robot.get_energy().has_enough_energy(costo){
        true=>{
            for c in comandi{
                let res=go(robot,world,c.to_owned());
                if res.is_err(){
                    return Err(res.err().unwrap());
                }
            }
            Ok(())
        },
        false=>Err(NotEnoughEnergy)
    };
}




//Helper functions:
fn my_position(robot:&impl Runnable,world:&World ){
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
fn generate_map() -> Vec<Vec<Tile>> {
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









//
//
//Here it is an example with a random generated world.
#[test]
fn random_example(){
    struct WorldGeneratorEample{
        size:usize,
    }
    impl WorldGeneratorEample{
        fn new(size:usize) -> Self {
            WorldGeneratorEample {size}
        }
    }
    impl Generator for WorldGeneratorEample{
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
            let mut rng = rand::thread_rng();
            let mut map: Vec<Vec<Tile>> = Vec::new();
            // Initialize the map with default tiles
            for _ in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for _ in 0..self.size {
                    let i_tiletype = rng.gen_range(0..=9);
                    let i_content = rng.gen_range(0..=2);
                    let elevation=rng.gen_range(0..=9);
                    let mut tile_type = match i_tiletype {
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
                    tile_type=Grass;
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

            (map, (0, 0), environmental_conditions, max_score,Option::None)
        }
    }


    struct MyRobot(Robot);

    impl Runnable for MyRobot{
        fn process_tick(&mut self, world: &mut World) {
            let (map,dimension,(x_robot,y_robot))=debug(self,world);
            for i in &map{
                for j in i{
                    print!(" |{:?} c={}| ",j.tile_type,j.content);
                }
                println!();
            }
            println!("\n");
            for i in &map{
                for j in i{
                    print!(" |{}| ",j.elevation);
                }
                println!();
            }
            my_position(self,world);
            let directions=[Right,Right,Right,Down,Left,Up];
            let result=attuatore(&directions,10,self,world);
            match result{
                Ok(T)=>my_position(self,world),
                Err(er)=>println!("We got this error:{:?}",er)
            }
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
    let mut generator:WorldGeneratorEample=WorldGeneratorEample{size:4};

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













//
//
//Here we have an example for a generated map (debbuging purpose).
#[test]
fn generated_example(){
    struct WorldGenerator{
        size:usize,
    }
    impl WorldGenerator{
        fn new(size:usize) -> Self {
            WorldGenerator {size}
        }
    }
    impl Generator for WorldGenerator{
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
            let map=generate_map();
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

            let max_score = rand::random::<f32>();

            (map, (0, 0), environmental_conditions, max_score,Option::None)
        }
    }


    struct MyRobot(Robot);

    impl Runnable for MyRobot{
        fn process_tick(&mut self, world: &mut World) {
            let (map,dimension,(x_robot,y_robot))=debug(self,world);
            for i in &map{
                for j in i{
                    print!(" |{:?} c={}| ",j.tile_type,j.content);
                }
                println!();
            }
            println!("\n");
            for i in &map{
                for j in i{
                    print!(" |{}| ",j.elevation);
                }
                println!();
            }
            my_position(self,world);
            let directions=[Right,Right,Right,Down,Left,Up];
            let result=attuatore(&directions,10,self,world);
            match result{
                Ok(T)=>my_position(self,world),
                Err(er)=>println!("We got this error:{:?}",er)
            }
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
    let r = MyRobot(Robot::new());
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];
    let mut generator=WorldGenerator{size:4};

    let run = Runner::new(Box::new(r), &mut generator, tools);
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}