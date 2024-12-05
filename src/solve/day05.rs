use std::collections::HashMap;

fn parse_input(input: String) -> (Vec<(String, String)>, Vec<Vec<String>>) {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    let mut split = false;
    for line in input.lines() {
        if line == "" {
            split = true;
            continue;
        }
        if !split {
            let sp: Vec<&str> = line.split("|").collect();
            list_1.push((sp[0].to_string(), sp[1].to_string()));
        } else {
            let sp: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
            list_2.push(sp);
        }
    }

    return (list_1, list_2);
}

fn order_line(line: Vec<String>, order: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut ordered_line = Vec::new();

    for num in line {
        let mut nums = Vec::new();
        match order.get(&num) {
            Some(v) => {
                nums = v.to_vec();
            }
            None => {}
        }
        for (i, elem) in ordered_line.clone().into_iter().enumerate() {
            if nums.contains(&elem) {
                ordered_line.insert(i, num.clone());
                break;
            } else if i == ordered_line.len() - 1 {
                ordered_line.push(num.clone());
                break;
            }
        }
        if ordered_line.len() == 0 {
            ordered_line.push(num);
        }
    }

    return ordered_line;
}
pub fn solve_day5_1(input: String) -> i64 {
    let (list_1, list_2) = parse_input(input);
    let mut ans: i64 = 0;

    let mut order: HashMap<String, Vec<String>> = HashMap::new();

    for (f, s) in list_1 {
        if order.contains_key(&f) {
            if let Some(v) = order.get_mut(&f) {
                v.push(s);
            }
        } else {
            order.insert(f, vec![s]);
        }
    }

    for line in list_2 {
        let mut wrong = false;
        let mut p_nums = Vec::new();
        'outer: for num in line.iter() {
            for &p in p_nums.iter() {
                if order.contains_key(num) {
                    if order.get(num).unwrap().contains(p) {
                        wrong = true;
                        break 'outer;
                    }
                }
            }
            p_nums.push(num);
        }
        if !wrong {
            let idx = line.len() / 2;
            ans += line[idx].parse::<i64>().unwrap();
        }
    }
    return ans;
}

pub fn solve_day5_2(input: String) -> i64 {
    let (list_1, list_2) = parse_input(input);
    let mut ans: i64 = 0;

    let mut order: HashMap<String, Vec<String>> = HashMap::new();

    for (f, s) in list_1.iter() {
        if order.contains_key(f) {
            if let Some(v) = order.get_mut(f) {
                v.push(s.to_string());
            }
        } else {
            order.insert(f.to_string(), vec![s.to_string()]);
        }
    }

    for line in list_2 {
        let mut ordered_line = Vec::new();
        let mut wrong = false;
        let mut p_nums = Vec::new();
        for num in line.iter() {
            for &p in p_nums.iter() {
                if order.contains_key(num) {
                    if order.get(num).unwrap().contains(p) {
                        wrong = true;
                        ordered_line = order_line(line.clone(), order.clone());
                    }
                }
            }
            p_nums.push(num);
        }
        if wrong {
            let idx = ordered_line.len() / 2;
            ans += ordered_line[idx].parse::<i64>().unwrap();
        }
    }
    return ans;
}
