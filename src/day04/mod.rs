use std::cmp::Ordering;

fn six_digits(value: u32) -> [u32; 6] {
    [
        value % 10,
        (value / 10) % 10,
        (value / 100) % 10,
        (value / 1000) % 10,
        (value / 10000) % 10,
        (value / 100_000) % 10,
    ]
}

#[allow(dead_code)]
fn is_password_v1(value: u32) -> bool {
    let digits = six_digits(value);

    let mut has_adjacent_matching_digits = false;

    for i in 1..digits.len() {
        let [prev, curr] = [digits[i - 1], digits[i]];

        match prev.cmp(&curr) {
            Ordering::Less => return false,
            Ordering::Equal => {
                has_adjacent_matching_digits = true;
            }
            _ => {}
        }
    }

    has_adjacent_matching_digits
}

#[allow(dead_code)]
fn is_password_v2(value: u32) -> bool {
    let digits = six_digits(value);

    let mut has_adjacent_matching_digits = false;

    for i in 1..digits.len() {
        let [prev, curr] = [digits[i - 1], digits[i]];
        if prev < curr {
            return false;
        } else if !has_adjacent_matching_digits
            && prev == curr
            && (i == 1 || digits[i - 2] != prev)
            && (i == digits.len() - 1 || digits[i + 1] != curr)
        {
            has_adjacent_matching_digits = true;
        }
    }

    has_adjacent_matching_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert!(is_password_v1(111_111));
        assert!(!is_password_v1(123_789));
        assert!(!is_password_v1(223_450));

        assert_eq!((152_085..=670_283).filter(|value| is_password_v1(*value)).count(), 1764);
    }

    #[test]
    fn part2() {
        assert!(is_password_v2(111_122));
        assert!(!is_password_v2(111_222));
        assert!(is_password_v2(112_233));
        assert!(!is_password_v2(123_444));

        assert_eq!((152_085..=670_283).filter(|value| is_password_v2(*value)).count(), 1196);
    }
}
