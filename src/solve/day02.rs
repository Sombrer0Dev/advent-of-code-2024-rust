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
            println!("{:?}", line);
            let s = safe(line.to_vec(), false);
            println!("{s}");
            return s;
        })
        .count()
        .try_into()
        .unwrap()
}

fn safe(line: Vec<i64>, first: bool) -> bool {
    let mut order: bool = false;
    for (i, v) in line.iter().enumerate() {
        if i == 0 {
            order = line[1] > line[0]
        } else {
            let p_v = line[i - 1];
            let dist = (p_v - v).abs();
            if dist < 1 || dist > 3 {
                if first {
                    for i in 0..line.len() {
                        let mut l_line = line.clone();
                        l_line.remove(i);
                        if safe(l_line, false) {
                            return true;
                        }
                    }
                    return false;
                } else {
                    return false;
                }
            }
            if order != (*v > p_v) {
                if first {
                    for i in 0..line.len() {
                        let mut l_line = line.clone();
                        l_line.remove(i);
                        if safe(l_line, false) {
                            return true;
                        }
                    }
                    return false;
                } else {
                    return false;
                }
            }
        }
    }
    return true;
}
pub fn solve_day2_2(input: String) -> i64 {
    parse_input(input)
        .into_iter()
        .filter(|line| {
            let s = safe(line.to_vec(), true);
            println!("{:?}", line);
            println!("{s}");
            return s;
        })
        .count()
        .try_into()
        .unwrap()
}
