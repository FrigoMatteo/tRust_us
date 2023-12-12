use std::collections::HashMap;
use robotics_lib::world::tile::Content::Rock;
use crate::{
    robotics_lib::world::tile::{Tile,Content},
    robotics_lib::world::environmental_conditions::EnvironmentalConditions,
    robotics_lib::world::environmental_conditions::WeatherType::*,
    robotics_lib::runner::Robot,
    robotics_lib::runner::Runnable,
    robotics_lib::runner::backpack::BackPack,
    robotics_lib::runner::Runner,
    robotics_lib::event::events::Event,
    robotics_lib::energy::Energy,
    robotics_lib::world::coordinates::Coordinate,
    robotics_lib::world::world_generator::Generator,
    robotics_lib::world::World,
    robotics_lib::interface::{debug, Direction::*},
};
use crate::tools::actuator::actuator;
use crate::tools::gps::Command::Control;
use crate::tools::gps::Goal:: Resource;
use crate::tools::gps::gps;
use crate::tools_test::{generate_map, my_position};


#[test]
fn show_example1(){
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
            let map=debug(self,world);
            for i in &map.0{
                for j in i{
                    print!(" |{:?} c={}| ",j.tile_type,j.content);
                }
                println!();
            }
            println!("\n");
            for i in &map.0{
                for j in i{
                    print!(" |{}| ",j.elevation);
                }
                println!();
            }
            let directions=[Control(Down), Control(Down), Control(Down), Control(Right), Control(Right), Control(Left), Control(Left), Control(Up), Control(Up), Control(Up)];
            let r=actuator(&directions,10,self,world);
            match r{
                Ok(_)=>println!("Done"),
                Err(_)=>println!("Error"),
            }
            my_position(self,world);
            if let Some(i) = gps(self, Resource(Rock(1)), world, None) {
                println!("{:?}", i);
                let directions=i.0.as_slice();
                let cost=i.1;
                let res=actuator(directions,cost,self,world);
                println!("{:?}", res);
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
    let mut generator=WorldGenerator::new(4);

    let run = Runner::new(Box::new(r), &mut generator);
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}