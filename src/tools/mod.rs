use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::World;
use crate::tools::actuator::actuator;
use crate::tools::gps::{Goal, gps};

pub (crate) mod actuator;
pub mod gps;



pub fn research(dest: Goal, opt_teleports: Option<&[(usize, usize)]>, robot:&mut impl Runnable, world:&mut World) -> Result<(), LibError> {
    let result=gps(robot,dest,world,opt_teleports);
    match result {
        Some(t)=>actuator(t.0.as_slice(),t.1,robot,world),
        None=>Err(LibError::OperationNotAllowed),
    }
}