fn parse_input(input: String) -> Vec<Vec<char>> {
    let mut list = Vec::new();
    let mut size: usize = 0;

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            size = line.len() + 6;
            list.push(".".repeat(size).chars().collect());
            list.push(".".repeat(size).chars().collect());
            list.push(".".repeat(size).chars().collect());
        }
        let mut t_list: Vec<char> = vec!['.', '.', '.'];
        let t_char: Vec<char> = line.chars().collect();
        t_list.extend(t_char);
        t_list.extend(vec!['.', '.', '.']);
        list.push(t_list);
    }
    list.push(".".repeat(size).chars().collect());
    list.push(".".repeat(size).chars().collect());
    return list;
}

/*
S..S..S
.A.A.A.
..MMM..
SAMXMAS
..MMM..
.A.A.A.
S..S..S
*/
const DIRECTIONS: [(i8, i8); 8] = [
    //(x, y)
    (1, 0),   // -> 0
    (-1, 0),  // <- 1
    (0, 1),   // ^  2
    (0, -1),  // v  3
    (1, 1),   // /^ 4
    (1, -1),  // \v 5
    (-1, 1),  // \^ 6
    (-1, -1), // /v 7
];

fn find_xmas(x: usize, y: usize, list: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;

    for &(p_x, p_y) in DIRECTIONS.iter() {
        if list[y.wrapping_add_signed(p_y.into())][x.wrapping_add_signed(p_x.into())] == 'M'
            && list[y.wrapping_add_signed((p_y * 2).into())]
                [x.wrapping_add_signed((p_x * 2).into())]
                == 'A'
            && list[y.wrapping_add_signed((p_y * 3).into())]
                [x.wrapping_add_signed((p_x * 3).into())]
                == 'S'
        {
            count += 1
        }
    }
    return count;
}

/*
M.S
.A.
M.S
*/
const X_MAS_DIR: [((i8, i8), (i8, i8)); 2] = [
    //(x, y)
    (
        (1, 1),   // /^ 4
        (-1, -1), // /v 7
    ),
    (
        (1, -1), // \v 5
        (-1, 1), // \^ 6
    ),
];
fn find_x_mas(x: usize, y: usize, list: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;

    for (d_1, d_2) in X_MAS_DIR.into_iter() {
        let x_1 = x.wrapping_add_signed(d_1.0.into());
        let x_2 = x.wrapping_add_signed(d_2.0.into());
        let y_1 = y.wrapping_add_signed(d_1.1.into());
        let y_2 = y.wrapping_add_signed(d_2.1.into());
        let char_1 = list[y_1][x_1];
        let char_2 = list[y_2][x_2];
        match (char_1, char_2) {
            ('M', 'S') | ('S', 'M') => count += 1,
            (_, _) => {}
        }
    }

    if count == 2 {
        return 1;
    }
    return 0;
}

pub fn solve_day4_1(input: String) -> i64 {
    let list = parse_input(input);
    let mut count = 0;

    for (y, line) in list.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'X' {
                count += find_xmas(x, y, &list);
            }
        }
    }

    return count;
}

pub fn solve_day4_2(input: String) -> i64 {
    let list = parse_input(input);
    let mut count = 0;

    for (y, line) in list.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'A' {
                count += find_x_mas(x, y, &list);
            }
        }
    }

    return count;
}
