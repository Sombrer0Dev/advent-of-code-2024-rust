use std::collections::HashMap;

fn parse_input(input: String) -> Vec<Vec<char>> {
    let mut list = Vec::new();

    for line in input.lines() {
        list.push(line.chars().collect());
    }
    return list;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

fn check_obstacle(x: i32, y: i32, dir: Direction, grid: Vec<Vec<char>>) -> bool {
    let mut m_x = x;
    let mut m_y = y;
    let mut t_dir = dir.clone();
    let mut map = HashMap::new();
    'outer: loop {
        match t_dir {
            Direction::North => loop {
                if m_y <= 0 {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y - 1][t_x];

                if face == '#' || face == 'O' {
                    t_dir = rotate(t_dir);
                    break;
                }
                if map.contains_key(&(t_y, t_x, t_dir.clone())) {
                    return true;
                } else {
                    map.insert((t_y, t_x, t_dir.clone()), true);
                }
                m_y -= 1;
            },
            Direction::South => loop {
                if m_y + 1 >= grid.len().try_into().unwrap() {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y + 1][t_x];

                if face == '#' || face == 'O' {
                    t_dir = rotate(t_dir);
                    break;
                }
                if map.contains_key(&(t_y, t_x, t_dir.clone())) {
                    return true;
                } else {
                    map.insert((t_y, t_x, t_dir.clone()), true);
                }
                m_y += 1;
            },
            Direction::West => loop {
                if m_x <= 0 {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y][t_x - 1];

                if face == '#' || face == 'O' {
                    t_dir = rotate(t_dir);
                    break;
                }
                if map.contains_key(&(t_y, t_x, t_dir.clone())) {
                    return true;
                } else {
                    map.insert((t_y, t_x, t_dir.clone()), true);
                }
                m_x -= 1;
            },
            Direction::East => loop {
                if m_x + 1 >= grid[0].len().try_into().unwrap() {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y][t_x + 1];

                if face == '#' || face == 'O' {
                    t_dir = rotate(t_dir);
                    break;
                }
                if map.contains_key(&(t_y, t_x, t_dir.clone())) {
                    return true;
                } else {
                    map.insert((t_y, t_x, t_dir.clone()), true);
                }
                m_x += 1;
            },
        }
    }
    return false;
}
fn move_guard_dir(x: &mut i32, y: &mut i32, grid: &mut Vec<Vec<char>>) -> i64 {
    let m_x = x.clone();
    let m_y = y.clone();
    let mut dir = Direction::North;
    let mut count = 0;
    let mut placed = HashMap::new();
    'outer: loop {
        match dir {
            Direction::North => loop {
                let t_y = *y as usize;
                let t_x = *x as usize;
                if *y == 0 {
                    break 'outer;
                }
                let face = grid[t_y - 1][t_x];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                } else {
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, Direction::North, grid.to_vec()) {
                        if !placed.contains_key(&(t_y, t_x)) {
                            placed.insert((t_y, t_x), true);
                            count += 1;
                        }
                    }
                    grid[t_y][t_x] = '.';
                }
                *y -= 1;
            },
            Direction::South => loop {
                let t_y = *y as usize;
                let t_x = *x as usize;
                if *y + 1 == grid.len().try_into().unwrap() {
                    break 'outer;
                }
                let face = grid[t_y + 1][t_x];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                } else {
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, Direction::North, grid.to_vec()) {
                        if !placed.contains_key(&(t_y, t_x)) {
                            placed.insert((t_y, t_x), true);
                            count += 1;
                        }
                    }
                    grid[t_y][t_x] = '.';
                }
                *y += 1;
            },
            Direction::West => loop {
                let t_y = *y as usize;
                let t_x = *x as usize;
                if *x == 0 {
                    break 'outer;
                }
                let face = grid[t_y][t_x - 1];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                } else {
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, Direction::North, grid.to_vec()) {
                        if !placed.contains_key(&(t_y, t_x)) {
                            placed.insert((t_y, t_x), true);
                            count += 1;
                        }
                    }
                    grid[t_y][t_x] = '.';
                }
                *x -= 1;
            },
            Direction::East => loop {
                let t_y = *y as usize;
                let t_x = *x as usize;
                if *x + 1 == grid[0].len().try_into().unwrap() {
                    break 'outer;
                }
                let face = grid[t_y][t_x + 1];
                if face == '#' {
                    dir = rotate(dir);
                    break;
                } else {
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, Direction::North, grid.to_vec()) {
                        if !placed.contains_key(&(t_y, t_x)) {
                            placed.insert((t_y, t_x), true);
                            count += 1;
                        }
                    }
                    grid[t_y][t_x] = '.';
                }
                *x += 1;
            },
        }
    }
    return count;
}

pub fn solve_day6_2(input: String) -> i64 {
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
    let mut t_x = x;
    let mut t_y = y;

    let ans = move_guard_dir(&mut t_x, &mut t_y, &mut input);
    return ans;
}
