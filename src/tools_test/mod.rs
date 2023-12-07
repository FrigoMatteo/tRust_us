use std::collections::{BinaryHeap, HashMap};
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
    robotics_lib::utils::{go_allowed, LibError, calculate_cost_go_with_environment,LibError::NotEnoughEnergy},
    robotics_lib::interface::{robot_map,Direction,Tools,where_am_i,craft, debug, destroy, go, look_at_sky, teleport, Direction::*},
};
use strum::IntoEnumIterator;
use rand::Rng;
use std::cmp::Ordering;


//Attutatore modified
pub fn attuatore (comandi: &[Direction], costo:usize, robot: &mut impl Runnable,world: &mut World) -> Result<(), LibError>{
    return match robot.get_energy().has_enough_energy(costo){
        true=>{
            for c in comandi{
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

fn gps (
    robot: &impl Runnable,
    destination: (usize,usize),
    world: &World,
) -> Option<(Vec<Direction>, usize)>{

    let map = robot_map(world);
    if map.is_none() { return Option::None; }
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

    if !costs.contains_key(&destination) { return Option::None; }

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










//Helper functions:
fn my_position(robot:&impl Runnable,world:&World ){
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
fn generate_map() -> Vec<Vec<Tile>> {
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
    for i in &map{
        for j in i{
            print!(" |{:?} C:{}| ",j.tile_type,j.content);
        }
        println!();
    }

    println!("\n\n");

    for i in &map{
        for j in i{
            print!(" |{}| ",j.elevation);
        }
        println!();
    }
    map
}









//
//
//Here it is an example with a random generated world.
#[test]
fn random_example(){
    struct WorldGeneratorEample{
        size:usize,
    }
    impl WorldGeneratorEample{
        fn new(size:usize) -> Self {
            WorldGeneratorEample {size}
        }
    }
    impl Generator for WorldGeneratorEample{
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
            let mut rng = rand::thread_rng();
            let mut map: Vec<Vec<Tile>> = Vec::new();
            // Initialize the map with default tiles
            for _ in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for _ in 0..self.size {
                    let i_tiletype = rng.gen_range(0..=9);
                    let i_content = rng.gen_range(0..=2);
                    let elevation=rng.gen_range(0..=9);
                    let mut tile_type = match i_tiletype {
                        | 0 => DeepWater,
                        | 1 => ShallowWater,
                        | 2 => Sand,
                        | 3 => Grass,
                        | 4 => Street,
                        | 5 => Hill,
                        | 6 => Mountain,
                        | 7 => Snow,
                        | 8 => Lava,
                        | 9 => Teleport(false),
                        | _ => Grass,
                    };
                    tile_type=Grass;
                    let content = match i_content {
                        | 0 => Rock(4),
                        | 1 => Tree(2),
                        | 2 => Garbage(2),
                        | 3 => Fire,
                        | 4 => Coin(2),
                        | 5 => Bin(2..3),
                        | 6 => Crate(2..3),
                        | 7 => Bank(3..54),
                        | 8 => Content::Water(20),
                        | 9 => Content::None,
                        | 10 => Fish(3),
                        | 11 => Market(20),
                        | 12 => Content::Building,
                        | 13 => Content::Bush(2),
                        | 14 => Content::JollyBlock(2),
                        | 15 => Content::Scarecrow,
                        | _ => Content::None,
                    };
                    //let content=None;
                    row.push(Tile {
                        tile_type,
                        content,
                        elevation,
                    });
                }
                map.push(row);
            }
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

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
            my_position(self,world);
            let res=gps(self,(1,1),world);

            /*match result{
                Ok(T)=>my_position(self,world),
                Err(er)=>println!("We got this error:{:?}",er)
            }*/
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
    //We create the robot
    let r = MyRobot(Robot::new());
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];
    //We create the generator
    let mut generator:WorldGeneratorEample=WorldGeneratorEample{size:4};

    //We create the runner(Where we call the generator.gen()
    //We create and position our robot
    //Return the instaziation of the robot
    let run = Runner::new(Box::new(r), &mut generator, tools);
    // We initialize the robot
        match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }
}













//
//
//Here we have an example for a generated map (debbuging purpose).
#[test]
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
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

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
            my_position(self,world);
            let res=gps(self,(1,1),world);
            /*let directions=[Right,Right,Right,Down,Left,Up];
            let result=attuatore(&directions,10,self,world);
            match result{
                Ok(T)=>my_position(self,world),
                Err(er)=>println!("We got this error:{:?}",er)
            }*/
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