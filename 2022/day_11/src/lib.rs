use std::collections::HashMap;
use std::str::FromStr;
use std::{fs::read_to_string, string::ParseError};

#[derive(Debug)]
struct Monkey {
    id: i32,
    inspected: u64,
    items: Vec<u64>,
    operation_func: Operation,
    test_value: u64,
    true_monkey_id: i32,
    false_monkey_id: i32,
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().split(' ').collect::<Vec<&str>>();
        match s.as_slice() {
            ["old", "+", value] => Ok(Operation::Add(value.parse::<u64>().unwrap())),
            ["old", "*", "old"] => Ok(Operation::Square),
            ["old", "*", value] => Ok(Operation::Multiply(value.parse::<u64>().unwrap())),
            _ => panic!(),
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let id = lines
            .next()
            .unwrap()
            .split_at(6)
            .1
            .trim()
            .split(':')
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(|item| item.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let operation_func = lines
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<Operation>()
            .unwrap();
        let test_value = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();
        let true_monkey_id = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let false_monkey_id = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        Ok(Monkey {
            id,
            inspected: 0,
            items,
            operation_func,
            test_value,
            true_monkey_id,
            false_monkey_id,
        })
    }
}

impl Monkey {
    fn inspect(&mut self) {
        self.inspected += self.items.len() as u64;
        for item in self.items.iter_mut() {
            match self.operation_func {
                Operation::Square => *item = item.pow(2),
                Operation::Add(x) => *item += x,
                Operation::Multiply(x) => *item *= x,
            };
        }
    }
}

pub fn monkey_business(path: &str, rounds: i32, calming: u64) -> u64 {
    let input = read_to_string(path).unwrap();
    let mut monkeys = input
        .split("\n\n")
        .map(|x| x.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();
    let mut thrown_items: HashMap<i32, Vec<u64>> = HashMap::new();
    for i in 0..monkeys.len() {
        thrown_items.insert(i as i32, Vec::new());
    }

    let common_div: u64 = monkeys.iter().map(|m| m.test_value).product();

    for _ in 0..rounds {
        for monkey in monkeys.iter_mut() {
            monkey
                .items
                .append(thrown_items.get_mut(&monkey.id).unwrap());
            thrown_items.get_mut(&monkey.id).unwrap().clear();
            monkey.inspect();
            while let Some(item) = monkey.items.pop() {
                let item = item / calming;
                let item = item % common_div;
                let target_monkey = match item % monkey.test_value {
                    0 => monkey.true_monkey_id,
                    _ => monkey.false_monkey_id,
                };
                thrown_items.get_mut(&target_monkey).unwrap().push(item);
            }
        }
    }
    monkeys.sort_by_key(|d| d.inspected);
    monkeys.reverse();
    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspected)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calming_monekys() {
        let result = monkey_business("example.txt", 20, 3);
        assert_eq!(result, 10605);
    }

    #[test]
    fn shit_hit_the_fan() {
        let result = monkey_business("example.txt", 10_000, 1);
        assert_eq!(result, 2713310158);
    }
}
