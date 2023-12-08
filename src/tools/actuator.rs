use robotics_lib::interface::{Direction, go, teleport};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::utils::LibError::NotEnoughEnergy;
use robotics_lib::world::World;
use crate::tools::gps::Command;

//Il mio è più bello scemoh

// più bello
pub fn actuator(
    commands: &[Command],
    cost: usize,
    robot: &mut impl Runnable,
    world: &mut World,
) -> Result<(), LibError>{
    // energy control
    if !robot.get_energy().has_enough_energy(cost) { Err(NotEnoughEnergy) }
    // work hours
    for c in commands {
        let res= match c {
            Command::D(dir) => go(robot, world, dir.to_owned()),
            Command::T(x, y) => teleport(robot, world, (*x,*y)),
        };

        if res.is_err(){ Err(res.err().unwrap()) }
    }
    Ok(())
}

// pub fn actuator(commands: &[Direction], cost: usize, robot: &mut impl Runnable, world: &mut World) -> Result<(), LibError>{
//     return match robot.get_energy().has_enough_energy(cost){
//         true=>{
//             for c in commands {
//                 let res=go(robot,world,c.to_owned());
//                 if res.is_err(){
//                     return Err(res.err().unwrap());
//                 }
//             }
//             Ok(())
//         },
//         false=>Err(NotEnoughEnergy)
//     };
// }