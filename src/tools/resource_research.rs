use std::collections::{BinaryHeap, HashMap};
use robotics_lib::interface::{Direction, robot_map};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::Content;
use robotics_lib::world::World;

pub fn resource_research(contents:Content,robot:&impl Runnable,world:&World)-> Option<HashMap<Content,(usize,usize)>>{
    let opt_map = robot_map(world);
    if opt_map.is_none() { return Option::None; }
    let map = opt_map.unwrap();
    for i in &map{
        for j in i{
            match j{
                Some(T)=>println!(" |{:?}| ",T.tile_type),
                None=>println!(" |None| "),
            }
        }
    }

    let start = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
    let mut costs : HashMap<(usize,usize),(Direction,usize)> = HashMap::new();
    //let mut to_visit = BinaryHeap::new();

    return Option::None;

}

struct Visit {
    coord: (usize, usize),
    previous: usize,
    distance: usize,
}