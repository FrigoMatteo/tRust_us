use std::collections::{BinaryHeap, HashMap};
use strum::IntoEnumIterator;
use rand::Rng;
use std::cmp::Ordering;
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
    robotics_lib::utils::{go_allowed, LibError, calculate_cost_go_with_environment,LibError::NotEnoughEnergy},
    robotics_lib::interface::{robot_map,Direction,Tools,where_am_i,craft, debug, destroy, go, look_at_sky, teleport, Direction::*},
};

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

