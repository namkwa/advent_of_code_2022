use std::fs;
fn main() {
    let input = fs::read_to_string("day_14/src/input.txt").unwrap();

    let mut input: Vec<&str> = input.lines().collect();
    input.push("673,173 -> 327,173");
    star1(input);
    //star2(&input);
}

fn star1(input: Vec<&str>) {
    let walls = parse_data(input);
    let (max_x, max_y, min_x) = get_max_min_x_y(&walls);
    let max_x = max_x - min_x + 1;
    let max_y = max_y + 1;
    let walls = update_walls_coordinates(walls, &min_x, &0);
    let mut cave = create_cave(&walls, max_x, max_y);
    let mut y = 0;
    let mut x = 500 - min_x;
    let mut total = 0;
    while cave[y][x] != 'o' {
        if x < max_x {
            if *cave.get(y + 1).unwrap().get(x).unwrap() == '.' {
                y += 1;
                mid += 1;
            } else if *cave.get(y + 1).unwrap().get(x - 1).unwrap() == '.' {
                y += 1;
                x -= 1;
                left += 1;
            } else if *cave.get(y + 1).unwrap().get(x + 1).unwrap() == '.' {
                y += 1;
                x += 1;
                right += 1;
            } else {
                cave[y][x] = 'o';
                y = 0;
                x = 500 - min_x;
                total += 1;
            }
        } else {
            break;
        }
    }
    print_cave(cave);
    println!("{}", total);
}

fn parse_data(input: Vec<&str>) -> Vec<Vec<(usize, usize)>> {
    let mut walls: Vec<Vec<(usize, usize)>> = Vec::new();
    for line in input {
        let mut wall: Vec<(usize, usize)> = Vec::new();
        let parsed_line: Vec<&str> = line.split(" -> ").collect();
        for coordinate in parsed_line {
            let parsed_coordinate = coordinate.split_once(",").unwrap();
            wall.push((
                parsed_coordinate.0.parse::<usize>().unwrap(),
                parsed_coordinate.1.parse::<usize>().unwrap(),
            ))
        }
        walls.push(wall);
    }
    return walls;
}

fn get_max_min_x_y(walls: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize) {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut min_x: usize = 1000;
    for wall in walls {
        for coordinates in wall {
            if coordinates.0 > max_x {
                max_x = coordinates.0;
            } else if coordinates.0 < min_x {
                min_x = coordinates.0
            }
            if coordinates.1 > max_y {
                max_y = coordinates.1;
            }
        }
    }
    return (max_x, max_y, min_x);
}

fn update_walls_coordinates(
    walls: Vec<Vec<(usize, usize)>>,
    shift_x: &usize,
    shift_y: &usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut updated_walls: Vec<Vec<(usize, usize)>> = Vec::new();
    for (idx, wall) in walls.iter().enumerate() {
        updated_walls.push(Vec::new());
        for coordinates in wall {
            updated_walls[idx].push((coordinates.0 - shift_x, coordinates.1 - shift_y));
        }
    }
    return updated_walls;
}

fn create_cave(walls: &Vec<Vec<(usize, usize)>>, max_x: usize, max_y: usize) -> Vec<Vec<char>> {
    let mut cave: Vec<Vec<char>> = vec![vec!['.'; max_x]; max_y];
    for wall in walls {
        for idx in 1..wall.len() {
            let coordinates0 = *wall.get(idx - 1).unwrap();
            let coordinates1 = *wall.get(idx).unwrap();
            let (max_wall_x, min_wall_x) = sort_tuple_x(&coordinates0, &coordinates1);
            let (max_wall_y, min_wall_y) = sort_tuple_y(&coordinates0, &coordinates1);
            for x in min_wall_x..=max_wall_x {
                for y in min_wall_y..=max_wall_y {
                    cave[y][x] = '#';
                }
            }
        }
    }
    return cave;
}

fn sort_tuple_x(v1: &(usize, usize), v2: &(usize, usize)) -> (usize, usize) {
    let max_wall_x = core::cmp::max(v1.0, v2.0);
    let min_wall_x = core::cmp::min(v1.0, v2.0);
    return (max_wall_x, min_wall_x);
}

fn sort_tuple_y(v1: &(usize, usize), v2: &(usize, usize)) -> (usize, usize) {
    let max_wall_y = core::cmp::max(v1.1, v2.1);
    let min_wall_y = core::cmp::min(v1.1, v2.1);
    return (max_wall_y, min_wall_y);
}

fn print_cave(cave: Vec<Vec<char>>) {
    let mut to_print = String::new();
    for line in cave {
        let mut current = String::new();
        for cell in line {
            current.push(cell);
        }
        current.push_str("\n");
        to_print.push_str(&current);
    }
    print!("{}", to_print);
}
