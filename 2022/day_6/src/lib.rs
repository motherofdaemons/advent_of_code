use std::collections::HashSet;

pub fn find_first_marker(data_stream: &str, marker_len: usize) -> usize {
    for (index, window) in data_stream.as_bytes().windows(marker_len).enumerate() {
        let mut unique = HashSet::new();
        if window.into_iter().all(|x| unique.insert(x)) {
            return index + marker_len;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first_marker_test_1() {
        let result = find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        assert_eq!(result, 7);
    }
    #[test]
    fn find_first_marker_test_2() {
        let result = find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        assert_eq!(result, 5);
    }
    #[test]
    fn find_first_marker_test_3() {
        let result = find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 4);
        assert_eq!(result, 6);
    }
    #[test]
    fn find_first_marker_test_4() {
        let result = find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
        assert_eq!(result, 10);
    }
    #[test]
    fn find_first_marker_test_5() {
        let result = find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
        assert_eq!(result, 11);
    }

    #[test]
    fn find_first_marker_test_6() {
        let result = find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
        assert_eq!(result, 19);
    }
    #[test]
    fn find_first_marker_test_7() {
        let result = find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
        assert_eq!(result, 23);
    }
    #[test]
    fn find_first_marker_test_8() {
        let result = find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 14);
        assert_eq!(result, 23);
    }
    #[test]
    fn find_first_marker_test_9() {
        let result = find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
        assert_eq!(result, 29);
    }
    #[test]
    fn find_first_marker_test_10() {
        let result = find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);
        assert_eq!(result, 26);
    }
}
