use std::collections::{BinaryHeap, HashMap, HashSet};
use robotics_lib;
use robotics_lib::interface::{Direction::*, Direction, go, look_at_sky, robot_map};
use robotics_lib::runner::{Robot, Runner};
use robotics_lib::utils::{go_allowed, LibError, calculate_cost_go_with_environment};
use robotics_lib::world::tile::Content;
use robotics_lib::world::World;

fn main() {

    println!("Hello, world!");
}

fn attuatore (comandi: &[Direction], mut robot: &Runner, mut world: &World) -> Result<(), LibError>{
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
}

// risorse: content, [0 = tutte, n = finisce il prima possbile]
// distanza:         [0 = tutte, n]
fn ricerca_risorse (risorse: &[(Content, u32)], distanza: u32, mut robot: Runner, mut world: World) {
    // metto in or delle condizioni di uscita che diachiaro prima di cominciare al bfs

}

fn dijkstra (start: (usize, usize), distance: usize, robot: &Runner, world: &World) -> Result<HashMap<(usize, usize), i32>, LibError> {
    let mut arr_cost = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    arr_cost.insert(start, 0 as i32);
    to_visit.push((start, 0 as i32));

    while let Some(((row, col), cost_now)) = to_visit.pop() {
        if arr_cost.get(&(row, col)).is_some() {
            continue;
        } else if cost_now > distance as i32 {
            break;
        }

        for direction in Direction::iter() {
            match go_allowed(robot, world, &direction)? {
                Ok(_) => {},
                Err(_) => continue
            }
            let new_cost = cost_now + cost(direction, robot, world);
            let vicino = get_coords_row_col((row, col), &direction);

            if arr_cost.get(&vicino).map_or(true, |&current| new_cost < current) {
                arr_cost.insert(vicino, new_cost);
                to_visit.push((vicino, new_cost));
            }
        }
    }

    Ok(arr_cost)
}

fn get_coords_row_col(from: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        | Direction::Up => (from.0 - 1, from.1),
        | Direction::Down => (from.0+ 1, from.1),
        | Direction::Left => (from.0, from.1 - 1),
        | Direction::Right => (from.0, from.1 + 1),
    }
}

fn cost (direction: Direction, robot: &Runner, world: &World) -> i32 {

    let (row, col) = get_coords_row_col((robot.get_coordinate().get_row(), robot.get_coordinate().get_col()), &direction);

    // Get tiles
    let map = robot_map(&world);
    let target_tile = &map[row][col];
    let current_tile = &map[robot.get_coordinate().get_row()][robot.get_coordinate().get_col()];
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
