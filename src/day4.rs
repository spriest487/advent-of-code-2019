use digits_iterator::*;

fn match_part_1(digits: &[u8]) -> bool {
    if !digits.windows(2).any(|w| w[0] == w[1]) {
        return false;
    }

    if digits.windows(2).any(|w| w[0] > w[1]) {
        return false;
    }

    true
}

fn match_part_2(digits: &[u8]) -> bool {
    if !match_part_1(digits) {
        return false;
    }

    let mut has_double_only = false;
    let mut i = 0;
    let mut last = 255;
    let mut last_count = 0;

    loop {
        if digits[i] != last {
            if last_count == 2 {
                has_double_only = true;
                break;
            }

            last = digits[i];
            last_count = 1;
        } else {
            last_count += 1;
        }

        if i == digits.len() - 1 {
            if last_count == 2 {
                has_double_only = true;
            }
            break;
        } else {
            i += 1;
        }
    }

    has_double_only
}

fn main() {
    let min = 372304;
    let max = 847060;

    let mut count_1 = 0;
    let mut count_2 = 0;

    let mut digits = Vec::new();
    for current in min..=max {
        digits.clear();
        digits.extend(current.digits());

        if match_part_1(&digits) {
            count_1 += 1;
        }

        if match_part_2(&digits) {
            count_2 += 1;
        }
    }

    println!("password count (part 1 rules): {}", count_1);
    println!("password count (part 2 rules): {}", count_2);
}