use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::World;
use crate::tools::actuator::actuator;
use crate::tools::gps::{Goal, gps};

pub (crate) mod actuator;
pub mod gps;


/// A function to get the best path and reach the specific position or the closest content.\
/// For the destination we use an ```enum Goal={Coordinates(usize,usize),Resource(Content)}```
///
/// # Arguments
/// - `dest `: Which is your objective, definend as **Goal**.
/// - `opt_teleports`: Coordinates of the teleports you found.
/// - `robot `: The robot that has to follow the path.
/// - `world `: The world that the robot is in.
///
/// # Returns
/// It returns a Result based whether the robot manage to find and reach the location.
pub fn research(dest: Goal, opt_teleports: Option<&[(usize, usize)]>, robot:&mut impl Runnable, world:&mut World) -> Result<(), LibError> {
    let result=gps(robot,dest,world,opt_teleports);
    match result {
        Some(t)=>actuator(t.0.as_slice(),t.1,robot,world),
        None=>Err(LibError::OperationNotAllowed),
    }
}