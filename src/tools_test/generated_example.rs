use std::collections::HashMap;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{debug, Tools,Direction::{Down, Left, Right, Up}};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions,WeatherType::Sunny};
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;
use robotics_lib::world::worldgenerator::Generator;
use crate::tools::actuator::actuator;
use crate::tools::gps::gps;
use crate::tools_test::{generate_map, my_position};

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
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny], 15, 12).unwrap();

            let max_score = rand::random::<f32>();

            (map, (0, 0), environmental_conditions, max_score,None)
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
            let directions=[Down,Down,Down,Right,Right,Left,Left,Up,Up,Up];
            let r= actuator(&directions, 10, self, world);
            my_position(self,world);
            let res= gps(self,(3,2),world);
            if res.is_some(){
                let i=res.unwrap();
                let directions=i.0.as_slice();
                let cost=i.1;
                let res=actuator(directions,cost,self,world);

            }
            my_position(self,world);
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
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}