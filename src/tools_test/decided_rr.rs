use std::collections::{BinaryHeap, HashMap};
use crate::{
    tools::{attuatore},
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
    robotics_lib::utils::{go_allowed, LibError, calculate_cost_go_with_environment,LibError::NotEnoughEnergy},
    robotics_lib::interface::{robot_view,robot_map,Direction,Tools,where_am_i,craft, debug, destroy, go, look_at_sky, teleport, Direction::*},
};
use strum::IntoEnumIterator;
use rand::Rng;
use std::cmp::Ordering;
use crate::tools_test::{my_position,generate_map, gps};
#[test]
fn generated_rr(){
    struct WorldGeneratorRR{
        size:usize,
    }
    impl WorldGeneratorRR{
        fn new(size:usize) -> Self {
            WorldGeneratorRR {size}
        }
    }
    impl Generator for WorldGeneratorRR{
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
            let directions=[Down,Down,Down,Right,Right,Left,Left,Up,Up,Up];
            let r=attuatore(&directions,10,self,world);
            my_position(self,world);
            let res=rr(Rock(4),self,world);
            println!("{:?}",res);
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
    let mut generator=WorldGeneratorRR{size:4};

    let run = Runner::new(Box::new(r), &mut generator, tools);
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}