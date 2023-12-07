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

/*pub fn attuatore (comandi: &[Direction], mut robot: &Runner, mut world: &World) -> Result<(), LibError>{
    for c in comandi.iter() {
        while match go(&mut robot, &mut world, *c.clone()) {
            Ok(_) => true,
            Err(error) => {
                match error {
                    LibError::NotEnoughEnergy => {
                        //faccio passare un tick
                        world.advance_time();
                    }
                    _ => Err(error)
                }
                false
            }
        }{}
    }
    Ok(())
}*/