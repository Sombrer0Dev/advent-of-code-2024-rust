fn parse_input(input: String) -> Vec<Vec<char>> {
    let mut list = Vec::new();

    for line in input.lines() {
        list.push(line.chars().collect());
    }
    return list;
}

#[derive(Debug)]
enum Direction {
    South,
    North,
    West,
    East,
}
fn rotate(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::East => Direction::South,
    }
}
fn move_guard(x: &mut i32, y: &mut i32, grid: &mut Vec<Vec<char>>) -> i64 {
    let mut dir = Direction::North;
    let mut count = 0;
    'outer: loop {
        match dir {
            Direction::North => loop {
                if *y == 0 {
                    break 'outer;
                }
                let t_y = *y as usize;
                let t_x = *x as usize;
                let face = grid[t_y - 1][t_x];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                }
                *y -= 1;
                if grid[t_y][t_x] != 'X' {
                    grid[t_y][t_x] = 'X';
                    count += 1;
                }
            },
            Direction::South => loop {
                if *y + 1 == grid.len().try_into().unwrap() {
                    break 'outer;
                }
                let t_y = *y as usize;
                let t_x = *x as usize;
                let face = grid[t_y + 1][t_x];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                }
                *y += 1;
                if grid[t_y][t_x] != 'X' {
                    grid[t_y][t_x] = 'X';
                    count += 1;
                }
            },
            Direction::West => loop {
                if *x == 0 {
                    break 'outer;
                }
                let t_y = *y as usize;
                let t_x = *x as usize;
                let face = grid[t_y][t_x - 1];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                }
                *x -= 1;
                if grid[t_y][t_x] != 'X' {
                    grid[t_y][t_x] = 'X';
                    count += 1;
                }
            },
            Direction::East => loop {
                if *x + 1 == grid[0].len().try_into().unwrap() {
                    break 'outer;
                }
                let t_y = *y as usize;
                let t_x = *x as usize;
                let face = grid[t_y][t_x + 1];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                }
                *x += 1;
                if grid[t_y][t_x] != 'X' {
                    grid[t_y][t_x] = 'X';
                    count += 1;
                }
            },
        }
    }
    return count;
}
pub fn solve_day6_1(input: String) -> i64 {
    let mut input = parse_input(input);
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == '^' {
                y = i as i32;
                x = j as i32;
            }
        }
    }

    move_guard(&mut x, &mut y, &mut input) + 1
}

pub fn solve_day6_2(input: String) -> i64 {
    let _ = input;

    let ans: i64;
    ans = 0;
    return ans;
}
