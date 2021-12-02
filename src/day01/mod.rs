use std::collections::VecDeque;

pub fn part_1<I: IntoIterator<Item = u16>>(input: I) -> u16 {
    measure_window(input, 1)
}

pub fn part_2<I: IntoIterator<Item = u16>>(input: I) -> u16 {
    measure_window(input, 3)
}

fn measure_window<I: IntoIterator<Item = u16>>(input: I, window_size: usize) -> u16 {
    let mut count = 0;
    let mut window = VecDeque::with_capacity(window_size);
    for depth in input {
        if window.len() == window_size {
            let prev = window.pop_front().unwrap();
            if depth > prev {
                count += 1;
            }
        }
        window.push_back(depth);
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    #[test]
    fn example_1_is_7() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = part_1(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_1_works() {
        let input = parse_file("src/day01/input.txt");
        let result = part_1(input);
        assert_eq!(result, 1752);
    }

    #[test]
    fn example_2_is_5() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = part_2(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn part_2_works() {
        let input = parse_file("src/day01/input.txt");
        let result = part_2(input);
        assert_eq!(result, 1781);
    }
}
