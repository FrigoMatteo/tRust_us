use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use robotics_lib::interface::{Direction, look_at_sky, robot_map};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::calculate_cost_go_with_environment;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;
use strum::IntoEnumIterator;

// A* algorithm based on a BinaryHeap sorted over f = g + h
// returns Option of a Vec of directions to get to a destination and the cost to get there or None if nor reachable
pub fn gps (
    robot: &impl Runnable,
    destination: (usize,usize),
    world: &World,
) -> Option<(Vec<Direction>, usize)>{

    let opt_map = robot_map(world);
    if opt_map.is_none() { return None; }
    let map = opt_map.unwrap();

    let start = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
    let mut costs : HashMap<(usize,usize),(Direction,usize)> = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    costs.insert(start, (Direction::Down, 0));
    to_visit.push(
        Visit {
            coord: start,
            g: 0,
            h: 0,
        }
    );

    while let Some (Visit{ coord, g, h: _h }) = to_visit.pop() {

        // exit
        if coord == destination { break; }

        for dir in Direction::iter() {

            // new neighbor
            let neighbor;
            // controls
            // border
            if match dir {
                | Direction::Up => coord.0 != 0,
                | Direction::Down => coord.0 != map.len() - 1,
                | Direction::Left => coord.1 != 0,
                | Direction::Right => coord.1 != map.len() - 1,
            } { neighbor = get_coords_row_col(coord, &dir, 1);
            } else { continue; }
            //non existent or not walkable
            if !(map[neighbor.0][neighbor.1].is_some() &&
                map[neighbor.0][neighbor.1].to_owned().unwrap().tile_type.properties().walk())
            { continue; }

            // new costs
            let new_g = cost_g(coord, neighbor, world, &map);
            let new_h = cost_h(neighbor, destination);

            // contained with better g, skip else update
            if costs.contains_key(&neighbor) && costs[&neighbor].1 < g { continue; } else { costs.insert(neighbor, (dir.clone(), g)); }

            // new !analysed element
            to_visit.push(
                Visit {
                    coord: neighbor,
                    g: g + new_g,
                    h: new_h,
                }
            )
        }
    }

    if !costs.contains_key(&destination) { return None; }

    // serve il backtracking
    let mut commands = Vec::new();
    let mut temp = destination;

    while temp != start {
        commands.push(costs[&temp].0.clone());
        temp = get_coords_row_col(temp, &costs[&temp].0, -1);
    }

    let len = commands.len();
    commands[0..len].reverse();
    Some((commands, costs[&destination].1))
}

fn get_coords_row_col(
    before: (usize, usize),
    direction: &Direction,
    delta: i32,
) -> (usize, usize) {
    match direction {
        | Direction::Up =>    ((before.0 as i32 - delta) as usize, before.1),
        | Direction::Down =>  ((before.0 as i32 + delta) as usize, before.1),
        | Direction::Left =>   (before.0,                         (before.1 as i32 - delta) as usize),
        | Direction::Right =>  (before.0,                         (before.1 as i32 + delta) as usize),
    }
}

fn cost_g(
    current_coord: (usize, usize),
    target_coord: (usize, usize),
    world: &World,
    map: &Vec<Vec<Option<Tile>>>,
) -> usize {
    // Get tiles
    let target_tile = &map[target_coord.0][target_coord.1].to_owned().unwrap();
    let current_tile = &map[current_coord.0][current_coord.1].to_owned().unwrap();

    // Init costs
    let mut base_cost = target_tile.tile_type.properties().cost();
    let mut elevation_cost = 0;

    // Get informations that influence the cost
    let environmental_conditions = look_at_sky(world);
    let new_elevation = target_tile.elevation;
    let current_elevation = current_tile.elevation;

    // Calculate cost
    base_cost = calculate_cost_go_with_environment(base_cost, environmental_conditions, target_tile.tile_type);

    // Consider elevation cost only if we are going from a lower tile to a higher tile
    if new_elevation > current_elevation {
        elevation_cost = (new_elevation - current_elevation).pow(2);
    }

    base_cost + elevation_cost
}
fn cost_h(
    neighbor: (usize, usize),
    destination: (usize, usize),
) -> usize {
    // manhattan
    (neighbor.0).abs_diff(destination.0) + (neighbor.1).abs_diff(destination.1)
}

struct Visit {
    coord: (usize, usize),
    g: usize,
    h: usize,
}
impl Visit {
    fn f (&self) -> usize {
        self.g + self.h
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f())
    }
}
impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Visit {}
impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.f().eq(&other.f())
    }
}