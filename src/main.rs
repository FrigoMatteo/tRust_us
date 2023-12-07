use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use strum::IntoEnumIterator;

use robotics_lib;
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
    robotics_lib::utils::{LibError, calculate_cost_go_with_environment,LibError::NotEnoughEnergy},
    robotics_lib::interface::{robot_map, Direction, Direction::*, Tools, where_am_i, debug, go, look_at_sky},
};

fn main() {

    println!("Hello, world!");
    generated_example();
}
fn generated_example(){
    struct WorldGenerator{
        size:usize,
    }
    impl WorldGenerator{
        fn new(size:usize) -> Self {
            WorldGenerator {size}
        }
    }
    impl Generator for WorldGenerator{
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
            let map=generate_map();
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny], 15, 12).unwrap();

            let max_score = rand::random::<f32>();

            (map, (0, 0), environmental_conditions, max_score,Option::None)
        }
    }


    struct MyRobot(Robot);

    impl Runnable for MyRobot{
        fn process_tick(&mut self, world: &mut World) {
            let (map,dimension,(x_robot,y_robot))=debug(self,world);
            for i in &map{
                for j in i{
                    print!(" |{:?} c={}| ",j.tile_type,j.content);
                }
                println!();
            }
            println!("\n");
            for i in &map{
                for j in i{
                    print!(" |{}| ",j.elevation);
                }
                println!();
            }
            let directions=[Down,Down,Down,Right,Right,Left,Left,Up,Up,Up];
            let r= actuator(&directions, 10, self, world);
            my_position(self,world);
            let res= gps(self,(2,2),world);
            println!("{:?}", res);
        }
        fn handle_event(&mut self, event: Event) {
            println!("{:?}", event);
        }

        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }

        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }

        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
    }
    let r = MyRobot(Robot::new());
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];
    let mut generator=WorldGenerator{size:4};

    let run = Runner::new(Box::new(r), &mut generator, tools);
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}

//Helper functions:
pub fn my_position(robot:&impl Runnable,world:&World ){
    let (view,(x,y))=where_am_i(robot,world);
    println!();
    println!("I am at x:{} and y:{}",x,y);
    println!("Thats what i see:");
    for i in &view{
        for j in i{
            if j.is_some(){
                print!("| {:?} c:{} |",j.to_owned().unwrap().tile_type,j.to_owned().unwrap().content);
            }else{
                print!("| None |");
            }
        }
        println!();
    }
    println!("I have this quantity of energy:{}\n",robot.get_energy().get_energy_level());
}

/*
*  MAP:
*    ______________________________________
*   |            |            |            |
*   |   Street   | Shallow W. |  DeepWater |
*   |    3 el    |   2 el     |    1 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Grass   |    Sand    |    Hill    |
*   |    3 el    |    2 el    |    4 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |   Lava     |    Snow    |  Mountain  |
*   |   3 el     |    7 el    |    9 el    |
*   |____________|____________|____________|
*
*
*/
pub fn generate_map() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:1,
        },
        Tile {
            tile_type: TileType::Street,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::ShallowWater,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: TileType::DeepWater,
            content: Content::None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:2,
        },
        Tile {
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::Sand,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: TileType::Hill,
            content: Content::None,
            elevation: 4,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:1,
        },
        Tile {
            tile_type: TileType::Lava,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::Snow,
            content: Content::None,
            elevation: 7,
        },
        Tile {
            tile_type: TileType::Mountain,
            content: Content::None,
            elevation: 9,
        },
    ]);
    map.push(vec![
        Tile{
            tile_type:TileType::Grass,
            content:Content::None,
            elevation:1,
        },
        Tile {
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: TileType::Sand,
            content: Content::Rock(20),
            elevation: 5,
        },
        Tile {
            tile_type: TileType::Mountain,
            content: Content::None,
            elevation: 4,
        },
    ]);
    map
}
pub fn actuator(commands: &[Direction], cost: usize, robot: &mut impl Runnable, world: &mut World) -> Result<(), LibError>{
    return match robot.get_energy().has_enough_energy(cost){
        true=>{
            for c in commands {
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

// più bello
// pub fn actuator(commands: &[Direction], cost: usize, robot: &mut impl Runnable, world: &mut World) -> Result<(), LibError>{
//     // energy control
//     if !robot.get_energy().has_enough_energy(cost) { Err(NotEnoughEnergy) }
//     // work hours
//     for c in commands {
//         let res=go(robot,world,c.to_owned());
//         if res.is_err(){
//             return Err(res.err().unwrap());
//         }
//     }
//     Ok(())
// }

// A* algorithm based on a BinaryHeap sorted over f = g + h
// returns Option of a Vec of directions to get to a destination and the cost to get there or None if nor reachable
pub fn gps (
    robot: &impl Runnable,
    destination: (usize,usize),
    world: &World,
) -> Option<(Vec<Direction>, usize)>{

    let opt_map = robot_map(world);
    if opt_map.is_none() { return Option::None; }
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

    if !costs.contains_key(&destination) { return Option::None; }

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