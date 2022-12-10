use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn should_move(&self, target: &Pos) -> bool {
        let x1 = self.x as i32;
        let x2 = target.x as i32;
        let y1 = self.y as i32;
        let y2 = target.y as i32;
        f64::sqrt(((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64) >= 2.0
    }

    fn move_towards(&mut self, target: &Pos) {
        match self.y.cmp(&target.y) {
            Ordering::Less => self.y += 1,
            Ordering::Greater => self.y -= 1,
            Ordering::Equal => (),
        }
        match self.x.cmp(&target.x) {
            Ordering::Less => self.x += 1,
            Ordering::Greater => self.x -= 1,
            Ordering::Equal => (),
        }
    }
}

pub fn rope_physics(path: &str, rope_len: usize) -> usize {
    let input = read_to_string(path).unwrap();
    let mut rope = vec![Pos::default(); rope_len];
    let mut visited = HashSet::new();
    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let distance = distance.trim().parse::<u32>().unwrap();
        for _ in 0..distance {
            // move the head
            if let Some(head) = rope.get_mut(0) {
                match direction {
                    "U" => head.y += 1,
                    "D" => head.y -= 1,
                    "R" => head.x += 1,
                    "L" => head.x -= 1,
                    _ => (),
                };
            }

            // move all the segments depending on their head
            for i in 1..rope_len {
                let head = rope.get(i - 1).unwrap().to_owned();
                let tail = rope.get_mut(i).unwrap();
                if tail.should_move(&head) {
                    tail.move_towards(&head);
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_segment_test() {
        let result = rope_physics("example.txt", 2);
        assert_eq!(result, 13);
    }

    #[test]
    fn ten_segment_test() {
        let result = rope_physics("example2.txt", 10);
        assert_eq!(result, 36);
    }
}
