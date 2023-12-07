use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use robotics_lib;
use robotics_lib::interface::{Direction, look_at_sky, robot_map};
use robotics_lib::runner::{Runnable};
use robotics_lib::utils::{calculate_cost_go_with_environment};
use robotics_lib::world::tile::{Tile};
use robotics_lib::world::World;
use strum::IntoEnumIterator;

fn main() {

    println!("Hello, world!");
}

fn gps (
    robot: &impl Runnable,
    destination: (usize,usize),
    world: &World,
) -> Option<(Vec<Direction>, usize)>{

    let map = robot_map(world);
    if map.is_none() { return None; }
    let map1 = map.unwrap();

    let start = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
    let mut costs : HashMap<(usize,usize),(Direction,usize)> = HashMap::new();
    let mut to_visit = BinaryHeap::new();
    
    costs.insert(start, (Direction::Up, 0));
    to_visit.push(
        Visit {
            vertex: start,
            parent: Direction::Up,
            cost: 0,
        }
    );

    while let Some (Visit{vertex, parent, cost}) = to_visit.pop() {
        // condizione di uscita
        if vertex == destination { break; }

        // se esite ed è migliore salto, se no aggiorno (visited diverso)
        if costs.contains_key(&vertex) && costs[&vertex].1 < cost { continue; } else { costs.insert(vertex, (parent, cost)); }

        for dir in Direction::iter() {
            // new neig
            let neighbor = get_coords_row_col(vertex, &dir);

            let map2 = map1.clone();
            //non existent
            if !(map1[neighbor.0][neighbor.1].is_some() && map1[neighbor.0][neighbor.1].to_owned().unwrap().tile_type.properties().walk()) { continue; }

            // new costs
            let new_c =  cost +
                                new_cost(neighbor, destination) +
                                cost_dest(vertex, neighbor, world, map2);

            // nuovo elemento
            to_visit.push(
                Visit {
                    vertex: neighbor,
                    parent: dir,
                    cost: new_c,
                }
            )
        }
    }
   
    if !costs.contains_key(&destination) { return None; }
    
    // serve il backtracking
    let mut path = Vec::new();
    let mut temp = destination;

    while temp != start {
        path.push(costs[&temp].0.clone());
        temp = get_coords_row_col(temp, &costs[&temp].0);
    }

    let len = path.len();
    path[0..len].reverse();
    Some((path, costs[&destination].1))
}
fn new_cost (
    neig: (usize,usize),
    dest: (usize,usize),
) -> usize {
    // manhattan
    (neig.0).abs_diff(dest.0) + (neig.1).abs_diff(dest.1)
}
fn get_coords_row_col(before: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        | Direction::Up =>    (before.0 -1, before.1   ),
        | Direction::Down =>  (before.0 +1, before.1   ),
        | Direction::Left =>  (before.0,    before.1 -1),
        | Direction::Right => (before.0,    before.1 +1),
    }
}
fn cost_dest (current_coord: (usize,usize), target_coord: (usize,usize), world: &World, map: Vec<Vec<Option<Tile>>>) -> usize {
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
struct Visit {
    vertex: (usize,usize),
    parent: Direction,
    cost: usize,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
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
        self.cost.eq(&other.cost)
    }
}

