use robotics_lib::interface::{destroy, go, teleport};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::utils::LibError::NotEnoughEnergy;
use robotics_lib::world::World;
use crate::tools::gps::Command;

//Il mio è più bello scemoh

/// A function which will command the robot to follow the specified path.
///
/// # Arguments
///
/// - `commands`: List of enum type Command, with the path that the robot has to follow.
/// - `cost `: The amount of the cost to arrive at the destination
/// - `robot `: The robot that has to follow the path.
/// - `world `: The world that the robot is in.
///
/// # Returns
/// gives back if the robot was able to do the following path.
pub (crate) fn actuator(
    commands: &[Command],
    cost: usize,
    robot: &mut impl Runnable,
    world: &mut World,
) -> Result<(), LibError>{
    // energy control
    if !robot.get_energy().has_enough_energy(cost) { return Err(NotEnoughEnergy); }
    // work hours
    for c in commands {
        let res = match c {
            Command::Control(dir) => { go(robot, world, dir.to_owned()).err() },
            Command::Teletransport(x, y) => { teleport(robot, world, (*x, *y)).err() },
            Command::Destroy(dir) => { destroy(robot, world, dir.clone()).err() },
        };

        if res.is_some() { return Err(res.unwrap()); }
    }
    Ok(())
}