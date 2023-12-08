use std::collections::HashMap;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{debug, Tools, Direction::{Down, Left, Right, Up}, teleport};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions,WeatherType::Sunny};
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::World;
use robotics_lib::world::worldgenerator::Generator;
use crate::tools::actuator::actuator;
use crate::tools::gps::Command::D;
use crate::tools::gps::gps;
use crate::tools_test::{generate_map, my_position};

pub (crate) fn generate_map_teleport() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile{
            tile_type:Grass,
            content:None,
            elevation:1,
        },
        Tile {
            tile_type:DeepWater,
            content: None,
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: None,
            elevation: 1,
        },
        Tile {
            tile_type: Teleport(false),
            content: None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:Grass,
            content:None,
            elevation:1,
        },
        Tile {
            tile_type: DeepWater,
            content: None,
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: None,
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:Grass,
            content:None,
            elevation:1,
        },
        Tile {
            tile_type: Teleport(false),
            content: None,
            elevation: 1,
        },
        Tile {
            tile_type: DeepWater,
            content: None,
            elevation: 1,
        },
        Tile {
            tile_type: Mountain,
            content: None,
            elevation: 6,
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
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: Rock(20),
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: None,
            elevation: 1,
        },
    ]);
    map
}



static mut FLAG1: bool =true;
static mut FLAG2: bool=false;
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
            let map=generate_map_teleport();
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny], 5, 12).unwrap();

            let max_score = rand::random::<f32>();

            (map, (0, 0), environmental_conditions, max_score,Option::None)
        }
    }


    struct MyRobot(Robot);

    impl Runnable for MyRobot{
        fn process_tick(&mut self, world: &mut World) {
            unsafe {
                if FLAG1 {
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
                    //Create the robot_map
                    let directions=[D(Down),D(Down),D(Right),D(Down),D(Right),D(Right),D(Up),D(Up),D(Up)];
                    let r= actuator(&directions, 10, self, world);
                    my_position(self,world);
                    let r=teleport(self,world,(2,1));
                    let directions=[D(Left),D(Up),D(Up)];
                    let r= actuator(&directions, 10, self, world);
                    my_position(self,world);
                }
                if FLAG2 {
                    println!("-----------------------------------------------------------------");
                    if let Some(i) = gps(self, (0,2), world, Some(&[(2,1), (0,3)])) {
                        println!("{:?}", i);
                        let res = actuator(i.0.as_slice(), i.1, self, world);
                        println!("{:?}", res);
                    }
                    my_position(self,world);
                }
            }



            /*let res= gps(self,(3,2),world);
            if res.is_some(){
                let i=res.unwrap();
                let directions=i.0.as_slice();
                let cost=i.1;
                let res=actuator(directions,cost,self,world);

            }
            my_position(self,world);*/
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
    let mut generator=WorldGenerator::new(4);

    let run = Runner::new(Box::new(r), &mut generator, tools);
    let n=0;
    let mut run =run.unwrap();
    let _=run.game_tick();
    unsafe { FLAG1 = false; }
    for _ in 0..40{
        let _=run.game_tick();
    }
    unsafe {FLAG2=true;}
    let _=run.game_tick();


}