use std::env;
use std::fs;

#[derive(PartialEq)]
enum Type {
    Guard,
    Open,
    Obstruction
}

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
    Nul
}

struct Coordinate {
    x: i32,
    y: i32,
    c: char,
    t: Type,
    d: Direction,
    v: bool
}

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();

    // check for filename
    if args.len() < 2 {
        println!("No file name specified!");
        return;
    }

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();
    
    let mut grid: Vec<Coordinate> = Vec::new();

    let mut x_bound: i32 = 0;
    let mut y_bound: i32 = 0;

    let mut guard = Coordinate{
        x: 0,
        y: 0,
        c: 'g',
        t: Type::Guard,
        d: Direction::North,
        v: false
    };

    let mut guard_start = Coordinate {
        x: 0,
        y: 0,
        c: 'g',
        t: Type::Guard,
        d: Direction::North,
        v: false
    };
        
    // load the grid
    let mut y: i32 = 0;
    for row in lines {
        let characters: Vec<char> = row.chars().collect();
        x_bound = characters.len() as i32;
        for (x, col) in characters.iter().enumerate() {
            let mut loc = Coordinate {
                x: x as i32,
                y: y,
                c: *col,
                t: Type::Open,
                d: Direction::Nul,
                v: false
            };
            
            let mut guard_loc = false;
            match col {
                '.' => loc.t = Type::Open,
                '#' => loc.t = Type::Obstruction,
                '^' => {
                    loc.t = Type::Open;
                    loc.c = '.';
                    guard_loc = true;
                    loc.v = true;
                },
                _ => println!("No Match for '{col}' found!"),
            }

            grid.push(loc);

            // set the guard
            if guard_loc {
                guard.x = x as i32;
                guard.y = y;
                
                guard_start.x = x as i32;
                guard_start.y = y;
            }
        }
        y += 1;
        y_bound = y;
    }

    let mut part1_sum = 1; // start at 1 for the first location the guard visited, the start
    let mut part2_sum = 0;

    // part1
    loop {
        //println!("Guard Loc: {}, {}", guard.x, guard.y);
        let moved: bool;
        let escaped: bool;
        (guard, grid, moved, escaped) = move_guard(guard, grid, x_bound, y_bound);

        if moved {
            for cord in &mut grid {
                if cord.x == guard.x && cord.y == guard.y {
                    if !cord.v {
                        part1_sum += 1;
                    }
                    cord.v = true;
                }
            }
        }

        if escaped {
            //println!("Guard Escaped!");
            break;
        }
    }

    //part2
    
    // start placing Obstructions
    let grid_length = grid.len();
    let mut guard2 = Coordinate {
        x: guard_start.x,
        y: guard_start.y,
        d: Direction::North,
        c: 'g',
        t: Type::Guard,
        v: false
    };

    for index in 0..grid_length {
        //print!("\x1B[2J\x1B[1;1H");
        //println!("{}/{}", index+1, grid_length);
        //reset the guard
        guard.x = guard_start.x;
        guard.y = guard_start.y;
        guard.d = Direction::North;

        guard2.x = guard_start.x;
        guard2.y = guard_start.y;
        guard2.d = Direction::North;

        if !grid[index].v {
            continue;
        }
        
        if grid[index].t == Type::Obstruction {
            //println!("Skipping obstruction already at x: {} y: {}", grid[index].x, grid[index].y);
            continue;
        }

        if grid[index].x == guard_start.x && grid[index].y == guard_start.y {
            //println!("Skipping guard start location!");
            continue;
        }

        if grid[index].t == Type::Open { // open spot
            grid[index].t = Type::Obstruction; // add Obstruction
        }

        loop {
            //println!("Guard Loc: {}, {}", guard.x, guard.y);
            let mut escaped: bool;
            (guard, grid, _, escaped) = move_guard(guard, grid, x_bound, y_bound);

            if escaped {
                break;
            }
            //println!("Guard Loc: {}, {}", guard.x, guard.y);

            (guard2, grid, _, escaped) = move_guard(guard2, grid, x_bound, y_bound);
            
            if escaped {
                break;
            }

            //println!("Guard 2 Loc: {}, {}", guard2.x, guard2.y);

            (guard2, grid, _, escaped) = move_guard(guard2, grid, x_bound, y_bound);

            if escaped {
                break;
            }

            //println!("Guard 2 Loc: {}, {}", guard2.x, guard2.y);

            // check for loop 
            if guard.x == guard2.x && guard.y == guard2.y && guard.d == guard2.d {
                part2_sum += 1;
                break;
            }
        }

        // set the grid type back to open
        grid[index].t = Type::Open;
    }

    println!("part1 sum: {part1_sum}");
    println!("part2 sum: {part2_sum}");
}

fn move_guard(mut guard: Coordinate, grid: Vec<Coordinate>, x_bound: i32, y_bound: i32) -> (Coordinate, Vec<Coordinate>, bool, bool) {
    // location to move to
    let x: i32;
    let y: i32;

    match guard.d {
        Direction::North => {
            x = guard.x;
            y = guard.y - 1;
        },
        Direction::South => {
            x = guard.x;
            y = guard.y + 1;
        },
        Direction::West => {
            x = guard.x - 1;
            y = guard.y;
        },
        Direction::East => {
            x = guard.x + 1;
            y = guard.y;
        },
        _ => return (guard, grid, false, false)
    }
    
    // check for escape
    if x < 0 || y < 0 || x == x_bound || y == y_bound {
        return (guard, grid, false, true);
    }

    // get the Coordinate of the location to move to
    let loc_to_move_to = &grid[((y * x_bound) + x) as usize];

    if loc_to_move_to.t == Type::Obstruction {
        // guard hits Obstruction and changes direction to the right
        match guard.d {
            Direction::North => guard.d = Direction::East,
            Direction::South => guard.d = Direction::West,
            Direction::West => guard.d = Direction::North,
            Direction::East => guard.d = Direction::South,
            _ => println!("Oops! Something went wrong!")
        }
        return (guard, grid, false, false);
    }

    // update guard location
    guard.x = x;
    guard.y = y;

    // moved but did not escape
    return (guard, grid, true, false);
}
