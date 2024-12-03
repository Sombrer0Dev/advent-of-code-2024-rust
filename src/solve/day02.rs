fn parse_input(input: String) -> Vec<Vec<i64>> {
    let mut list = Vec::new();

    for line in input.lines() {
        let line_parts = line.split_whitespace();
        let mut temp_vec = Vec::new();
        line_parts.for_each(|p| {
            temp_vec.push(p.parse::<i64>().unwrap());
        });
        list.push(temp_vec);
    }
    return list;
}
pub fn solve_day2_1(input: String) -> i64 {
    parse_input(input)
        .into_iter()
        .filter(|line| {
            let mut order: bool = false;
            for (i, v) in line.iter().enumerate() {
                if i == 0 {
                    order = line[1] > line[0]
                } else {
                    let p_v = line[i - 1];
                    let dist = (p_v - v).abs();
                    if dist < 1 || dist > 3 {
                        return false;
                    }
                    if order != (*v > p_v) {
                        return false;
                    }
                }
            }
            return true;
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_day2_2(input: String) -> i64 {
    let _ = input;

    let ans: i64;
    ans = 0;
    return ans;
}
