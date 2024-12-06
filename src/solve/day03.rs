use regex::Regex;
fn solve(input: String, trigger: bool) -> i64 {
    let mut do_mul = true;
    let reg = Regex::new(r"(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))").unwrap();
    let mut ans = 0;

    for m in reg.find_iter(input.as_str()) {
        match m.as_str() {
            "do()" => {
                do_mul = true;
            }
            "don't()" => {
                do_mul = false;
            }
            _ => {
                if !trigger || do_mul {
                    let num: Vec<i64> = m
                        .as_str()
                        .replace("mul(", "")
                        .replace(")", "")
                        .split(",")
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect();
                    ans += num[0] * num[1]
                }
            }
        }
    }
    return ans;
}
pub fn solve_day3_1(input: String) -> i64 {
    return solve(input, false);
}

pub fn solve_day3_2(input: String) -> i64 {
    return solve(input, true);
}
