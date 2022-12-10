use std::fs::read_to_string;

pub fn signal_strength(path: &str) -> i32 {
    let input = read_to_string(path).unwrap();
    let mut x = 1;
    let mut pc = 1;
    let mut result = 0;
    for line in input.lines() {
        let (instruction, value) = line.split_at(4);
        match instruction.trim() {
            "noop" => {
                pc += 1;
                if pc % 40 == 20 {
                    result += pc * x;
                }
            }
            "addx" => {
                pc += 1;
                if pc % 40 == 20 {
                    result += pc * x;
                }
                x += value.trim().parse::<i32>().unwrap();
                pc += 1;
                if pc % 40 == 20 {
                    result += pc * x;
                }
            }
            _ => unimplemented!(),
        }
    }
    result
}

pub fn draw_scan_lines(path: &str) -> String {
    let input = read_to_string(path).unwrap();
    let mut scan_lines = String::new();
    let mut pc = 1;
    let mut x = 1;
    for line in input.lines() {
        let (instruction, value) = line.split_at(4);
        match instruction.trim() {
            "noop" => {
                update_scan_lines(&mut scan_lines, pc, x);
                pc += 1;
            }
            "addx" => {
                update_scan_lines(&mut scan_lines, pc, x);
                pc += 1;
                update_scan_lines(&mut scan_lines, pc, x);
                x += value.trim().parse::<i32>().unwrap();
                pc += 1;
            }
            _ => unimplemented!(),
        }
    }
    scan_lines
}

fn update_scan_lines(scan_lines: &mut String, pc: i32, x: i32) {
    let mut pc = pc % 40;
    if pc == 0 {
        pc = 40
    }
    let pixel = match x - pc >= -2 && x - pc <= 0 {
        true => '#',
        false => '.',
    };
    scan_lines.push(pixel);
    if pc % 40 == 0 {
        scan_lines.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = signal_strength("example.txt");
        assert_eq!(result, 13140);
    }

    #[test]
    fn pretty_picture() {
        let result = draw_scan_lines("example.txt");
        print!("{}", result);
        let expected = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
        assert_eq!(result, expected);
    }
}
