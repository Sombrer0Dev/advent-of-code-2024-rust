fn parse_input(input: String) -> (Vec<i64>, Vec<i64>) {
    let mut loc_1 = Vec::new();
    let mut loc_2 = Vec::new();

    for line in input.lines() {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(first), Ok(second)) =
            (line_parts[0].parse::<i64>(), line_parts[1].parse::<i64>())
        {
            loc_1.push(first);
            loc_2.push(second);
        }
    }

    return (loc_1, loc_2);
}

pub fn solve_day1_1(input: String) -> i64 {
    let (t_1, t_2) = parse_input(input);
    let mut loc1 = t_1;
    let mut loc2 = t_2;
    loc1.sort();
    loc2.sort();
    loc1.into_iter()
        .zip(loc2.into_iter())
        .map(|(f, s)| (f - s).abs())
        .sum()
}

pub fn solve_day1_2(input: String) -> i64 {
    let (t_1, t_2) = parse_input(input);
    let loc1 = t_1;
    let loc2 = t_2;

    loc1.into_iter()
        .map(|x| x * (loc2.iter().filter(|&&n| x == n)).count() as i64)
        .sum()
}
