mod day_1;
mod day_2;
mod day_3;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_1;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_day_1_example_part_1() {
        day_1::part_1("input/day_1_part_1_example.txt");
    }
    #[test]
    fn test_day_1_part_1() {
        day_1::part_1("input/day_1_input.txt");
    }
    #[test]
    fn test_day_1_example_part_2() {
        day_1::part_2("input/day_1_part_2_example.txt");
    }
    #[test]
    fn test_day_1_part_2() {
        day_1::part_2("input/day_1_input.txt");
    }
    #[test]
    fn test_day_1_part_2_non_regex() {
        day_1::day_1_part_2_non_regex("input/day_1_input.txt");
    }
    #[test]
    fn test_day_1_part_2_proto_non_regex() {
        day_1::day_1_part_2_non_regex("input/day_1_input_proto.txt");
    }
    #[test]
    fn test_day_1_part_2_proto() {
        day_1::part_2("input/day_1_input_proto.txt");
    }
    #[test]
    fn test_day_1_part_1_proto() {
        day_1::part_1("input/day_1_input_proto.txt");
    }
}
