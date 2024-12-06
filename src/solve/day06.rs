fn parse_input(input: String) -> Vec<Vec<char>> {
    let mut list = Vec::new();

    for line in input.lines() {
        list.push(line.chars().collect());
    }
    return list;
}

#[derive(Debug, Clone)]
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
    let mut met = false;
    'outer: loop {
        match t_dir {
            Direction::North => loop {
                if m_y == 0 {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y - 1][t_x];
                if face == 'O' {
                    if met {
                        println!("met_second");
                        return true;
                    }
                    t_dir = rotate(t_dir);
                    met = true;
                    break;
                }
                if face == '#' {
                    t_dir = rotate(t_dir);
                    break;
                }
                m_y -= 1;
            },
            Direction::South => loop {
                if m_y + 1 == grid.len().try_into().unwrap() {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y + 1][t_x];
                if face == 'O' {
                    if met {
                        println!("met_second");
                        return true;
                    }
                    t_dir = rotate(t_dir);
                    met = true;
                    break;
                }
                if face == '#' {
                    t_dir = rotate(t_dir);
                    break;
                }
                m_y += 1;
            },
            Direction::West => loop {
                if m_x == 0 {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y][t_x - 1];
                if face == 'O' {
                    if met {
                        println!("met_second");
                        return true;
                    }
                    t_dir = rotate(t_dir);
                    met = true;
                    break;
                }
                if face == '#' {
                    t_dir = rotate(t_dir);
                    break;
                }
                m_x -= 1;
            },
            Direction::East => loop {
                if m_x + 1 == grid[0].len().try_into().unwrap() {
                    break 'outer;
                }
                let t_y = m_y as usize;
                let t_x = m_x as usize;
                let face = grid[t_y][t_x + 1];
                if face == 'O' {
                    if met {
                        println!("met_second");
                        return true;
                    }
                    t_dir = rotate(t_dir);
                    met = true;
                    break;
                }
                if face == '#' {
                    t_dir = rotate(t_dir);
                    break;
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
                } else {
                    println!("{t_y}, {t_x}");
                    let t_char = grid[t_y - 1][t_x];
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, dir.clone(), grid.to_vec()) {
                        count += 1;
                        println!("{count}");
                        for line in grid.into_iter() {
                            for ch in line {
                                print!("{ch}");
                            }
                            println!();
                        }
                    }
                    grid[t_y - 1][t_x] = t_char;
                }
                *y -= 1;
                if grid[t_y][t_x] != 'O' {
                    grid[t_y][t_x] = 'X';
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
                } else {
                    println!("{t_y}, {t_x}");
                    let t_char = grid[t_y + 1][t_x];
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, dir.clone(), grid.to_vec()) {
                        count += 1;
                        println!("{count}");
                        for line in grid.into_iter() {
                            for ch in line {
                                print!("{ch}");
                            }
                            println!();
                        }
                    }
                    grid[t_y + 1][t_x] = t_char;
                }
                *y += 1;
                if grid[t_y][t_x] != 'O' {
                    grid[t_y][t_x] = 'X';
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
                } else {
                    println!("{t_y}, {t_x}");
                    let t_char = grid[t_y][t_x - 1];
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, dir.clone(), grid.to_vec()) {
                        count += 1;
                        println!("{count}");
                        for line in grid.into_iter() {
                            for ch in line {
                                print!("{ch}");
                            }
                            println!();
                        }
                    }
                    grid[t_y][t_x - 1] = t_char;
                }
                *x -= 1;
                if grid[t_y][t_x] != 'O' {
                    grid[t_y][t_x] = 'X';
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
                } else {
                    println!("{t_y}, {t_x}");
                    let t_char = grid[t_y][t_x + 1];
                    grid[t_y][t_x] = 'O';
                    if check_obstacle(m_x, m_y, dir.clone(), grid.to_vec()) {
                        count += 1;
                        println!("{count}");
                        for line in grid.into_iter() {
                            for ch in line {
                                print!("{ch}");
                            }
                            println!();
                        }
                    }
                    grid[t_y][t_x + 1] = t_char;
                }
                *x += 1;
                if grid[t_y][t_x] != 'X' {
                    grid[t_y][t_x] = 'X';
                }
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
    for line in input {
        for ch in line {
            print!("{ch}");
        }
        println!();
    }
    return ans;
}
