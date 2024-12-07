use std::env;
use std::fs;

#[derive(PartialEq)]
enum Type {
    Guard,
    Open,
    Obstruction
}

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
        d: Direction::Nul,
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
            let mut dir = Direction::Nul;
            match col {
                '.' => loc.t = Type::Open,
                '#' => loc.t = Type::Obstruction,
                '^' => {
                    loc.t = Type::Open;
                    dir = Direction::North;
                    loc.c = '.';
                    guard_loc = true;
                    loc.v = true;
                },
                '>' => {
                    loc.t = Type::Open;
                    dir = Direction::West;
                    loc.c = '.';
                    guard_loc = true;
                    loc.v = true;
                },
                'v' => {
                    loc.t = Type::Open;
                    dir = Direction::South;
                    loc.c = '.';
                    guard_loc = true;
                    loc.v = true;
                },
                '<' => {
                    loc.t = Type::Open;
                    dir = Direction::East;
                    loc.c = '.';
                    guard_loc = true;
                    loc.v = true;
                },
                _ => println!("No Match for '{col}' found!"),
            }

            grid.push(loc);

            // set the guard
            if guard_loc {
                guard = Coordinate {
                    x: x as i32,
                    y: y,
                    t: Type::Guard,
                    c: 'g',
                    d: dir,
                    v: false
                };
            }
        }
        y += 1;
        y_bound = y;
    }
   
    let mut part1_sum = 1; // start at 1 for the first location the guard visited, it's starting
    // location
    loop {
        println!("Guard Loc: {}, {}", guard.x, guard.y);
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
            println!("Guard Escaped!");
            break;
        }
    }

    println!("part1_sum: {part1_sum}");
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
    let loc_to_move_to = grid.iter().find(|l| l.x == x && l.y == y).unwrap();

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
