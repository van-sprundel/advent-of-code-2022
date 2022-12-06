use anyhow::Result;

fn main() -> Result<()> {
    println!("{}", move_crates(false));
    println!("{}", move_crates(true));
    Ok(())
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::<Vec<char>>::with_capacity(9);

    input.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| !item.is_ascii_whitespace())
            .for_each(|(i, item)| {
                if i >= stacks.len() {
                    let mut new_vec = Vec::with_capacity(10);
                    new_vec.push(item);
                    stacks.push(new_vec);
                } else {
                    stacks[i].push(item);
                }
            })
    });

    stacks
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (count, temp) = line[5..].split_once(" from ")?;
            let (from, to) = temp.split_once(" to ")?;
            Some(Instruction {
                count: count.parse::<usize>().ok()?,
                from: from.parse::<usize>().ok()?,
                to: to.parse::<usize>().ok()?,
            })
        })
}

fn move_crate(stacks: &mut [Vec<char>], instruction: &Instruction, cratemover_9001: bool) {
    let from_stack = stacks.get_mut(instruction.from - 1).unwrap();

    let mut crates = from_stack.split_off(from_stack.len() - instruction.count);

    if !cratemover_9001 {
        crates.reverse();
    }

    let to_stack = stacks.get_mut(instruction.to - 1).unwrap();
    to_stack.append(&mut crates);
}

fn move_crates(cratemover_9001: bool) -> String {
    let input = INPUT;
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let stacks = parse_crates(crates);
    let instructions = parse_instructions(instructions);

    let new_stacks = instructions.fold(stacks, |mut stacks, op| {
        move_crate(&mut stacks, &op, cratemover_9001);
        stacks
    });

    new_stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

const INPUT: &str = r#"                     [N]     [H]
[V]         [P] [T]     [V]     [P]
[W]     [C] [T] [S]     [H]     [N]
[T]     [J] [Z] [M] [N] [F]     [L]
[Q]     [W] [N] [J] [T] [Q] [R] [B]
[N] [B] [Q] [R] [V] [F] [D] [F] [M]
[H] [W] [S] [J] [P] [W] [L] [P] [S]
[D] [D] [T] [F] [G] [B] [B] [H] [Z]
 1   2   3   4   5   6   7   8   9

move 2 from 8 to 1
move 4 from 9 to 8
move 2 from 1 to 6
move 7 from 4 to 2
move 10 from 2 to 7
move 2 from 1 to 6
move 1 from 9 to 4
move 1 from 4 to 1
move 8 from 6 to 4
move 7 from 1 to 8
move 6 from 8 to 1
move 1 from 4 to 1
move 8 from 7 to 3
move 2 from 5 to 2
move 5 from 3 to 2
move 5 from 2 to 1
move 1 from 6 to 5
move 2 from 2 to 6
move 5 from 8 to 7
move 12 from 7 to 4
move 3 from 5 to 4
move 2 from 6 to 4
move 9 from 1 to 7
move 4 from 3 to 7
move 4 from 3 to 4
move 3 from 1 to 7
move 1 from 9 to 1
move 1 from 1 to 4
move 2 from 5 to 2
move 1 from 3 to 7
move 15 from 7 to 2
move 4 from 7 to 9
move 6 from 9 to 2
move 2 from 8 to 3
move 3 from 2 to 8
move 1 from 7 to 6
move 8 from 2 to 5
move 2 from 8 to 4
move 2 from 3 to 8
move 9 from 5 to 9
move 7 from 4 to 2
move 1 from 8 to 6
move 6 from 9 to 2
move 3 from 9 to 7
move 2 from 8 to 4
move 7 from 2 to 6
move 7 from 4 to 1
move 3 from 1 to 8
move 2 from 1 to 8
move 4 from 8 to 2
move 2 from 1 to 5
move 19 from 2 to 7
move 8 from 4 to 7
move 18 from 7 to 1
move 11 from 7 to 4
move 15 from 1 to 7
move 9 from 4 to 3
move 2 from 3 to 1
move 9 from 4 to 5
move 1 from 8 to 1
move 8 from 6 to 5
move 3 from 2 to 5
move 1 from 6 to 7
move 4 from 4 to 3
move 8 from 5 to 1
move 13 from 1 to 6
move 12 from 7 to 1
move 12 from 6 to 3
move 1 from 7 to 6
move 1 from 7 to 5
move 1 from 1 to 9
move 1 from 3 to 1
move 3 from 1 to 9
move 12 from 3 to 8
move 1 from 9 to 3
move 1 from 6 to 8
move 5 from 5 to 1
move 1 from 6 to 2
move 10 from 8 to 9
move 13 from 9 to 2
move 10 from 3 to 4
move 1 from 8 to 9
move 2 from 8 to 7
move 1 from 3 to 1
move 1 from 5 to 6
move 13 from 2 to 5
move 1 from 9 to 2
move 7 from 1 to 4
move 2 from 2 to 5
move 2 from 7 to 8
move 1 from 6 to 8
move 10 from 5 to 8
move 3 from 7 to 2
move 4 from 1 to 4
move 12 from 4 to 2
move 10 from 5 to 3
move 6 from 2 to 1
move 2 from 4 to 8
move 3 from 4 to 8
move 6 from 1 to 7
move 1 from 7 to 5
move 12 from 8 to 2
move 3 from 4 to 9
move 1 from 4 to 3
move 2 from 9 to 6
move 2 from 6 to 8
move 1 from 1 to 3
move 8 from 2 to 6
move 4 from 1 to 8
move 12 from 2 to 3
move 4 from 6 to 8
move 10 from 8 to 3
move 14 from 3 to 8
move 5 from 5 to 8
move 1 from 7 to 8
move 5 from 3 to 5
move 4 from 7 to 2
move 2 from 6 to 1
move 4 from 3 to 7
move 4 from 5 to 1
move 21 from 8 to 6
move 7 from 3 to 2
move 1 from 5 to 1
move 4 from 8 to 9
move 16 from 6 to 1
move 1 from 8 to 4
move 5 from 9 to 2
move 7 from 1 to 7
move 10 from 1 to 3
move 1 from 4 to 2
move 6 from 6 to 5
move 6 from 1 to 4
move 4 from 7 to 9
move 1 from 6 to 5
move 5 from 7 to 6
move 3 from 6 to 8
move 1 from 7 to 6
move 6 from 4 to 8
move 4 from 8 to 3
move 4 from 8 to 4
move 17 from 2 to 1
move 8 from 3 to 4
move 5 from 4 to 3
move 10 from 1 to 5
move 11 from 3 to 5
move 1 from 7 to 9
move 3 from 6 to 4
move 9 from 4 to 9
move 7 from 1 to 3
move 1 from 4 to 8
move 7 from 5 to 4
move 18 from 5 to 1
move 13 from 1 to 6
move 1 from 1 to 5
move 1 from 1 to 6
move 2 from 3 to 1
move 1 from 3 to 1
move 5 from 1 to 6
move 4 from 5 to 8
move 2 from 4 to 9
move 1 from 1 to 9
move 6 from 3 to 8
move 1 from 4 to 5
move 10 from 8 to 7
move 16 from 6 to 7
move 1 from 5 to 4
move 1 from 7 to 2
move 2 from 2 to 6
move 2 from 8 to 5
move 5 from 4 to 9
move 2 from 5 to 9
move 7 from 9 to 8
move 2 from 6 to 9
move 4 from 8 to 9
move 7 from 9 to 7
move 13 from 9 to 5
move 10 from 5 to 1
move 3 from 8 to 4
move 5 from 1 to 3
move 3 from 5 to 6
move 3 from 9 to 7
move 1 from 1 to 7
move 2 from 1 to 3
move 1 from 6 to 1
move 4 from 3 to 8
move 1 from 8 to 9
move 1 from 8 to 7
move 1 from 8 to 4
move 1 from 9 to 7
move 1 from 8 to 5
move 2 from 4 to 3
move 4 from 6 to 3
move 1 from 5 to 1
move 1 from 6 to 4
move 2 from 4 to 5
move 1 from 4 to 6
move 1 from 6 to 4
move 30 from 7 to 3
move 1 from 5 to 1
move 6 from 7 to 3
move 2 from 1 to 7
move 2 from 1 to 2
move 2 from 2 to 1
move 1 from 4 to 9
move 3 from 1 to 2
move 1 from 9 to 5
move 2 from 7 to 1
move 1 from 7 to 3
move 1 from 1 to 9
move 1 from 5 to 8
move 1 from 1 to 2
move 1 from 7 to 3
move 1 from 9 to 4
move 18 from 3 to 4
move 1 from 5 to 9
move 1 from 9 to 6
move 1 from 2 to 7
move 1 from 8 to 7
move 1 from 6 to 3
move 1 from 7 to 2
move 14 from 4 to 6
move 1 from 7 to 6
move 15 from 6 to 4
move 20 from 3 to 1
move 5 from 4 to 9
move 5 from 4 to 2
move 15 from 1 to 7
move 11 from 7 to 9
move 2 from 7 to 6
move 1 from 6 to 4
move 1 from 6 to 3
move 2 from 7 to 8
move 10 from 4 to 3
move 15 from 9 to 3
move 1 from 9 to 7
move 29 from 3 to 6
move 3 from 1 to 6
move 1 from 8 to 4
move 2 from 4 to 3
move 1 from 8 to 9
move 4 from 6 to 1
move 20 from 6 to 2
move 5 from 1 to 9
move 3 from 6 to 2
move 4 from 6 to 3
move 4 from 3 to 1
move 4 from 1 to 4
move 3 from 4 to 8
move 6 from 3 to 4
move 6 from 2 to 6
move 1 from 7 to 1
move 3 from 6 to 8
move 6 from 9 to 3
move 1 from 1 to 4
move 1 from 1 to 7
move 3 from 4 to 5
move 2 from 6 to 4
move 2 from 5 to 6
move 4 from 8 to 7
move 1 from 5 to 6
move 1 from 8 to 4
move 1 from 8 to 4
move 2 from 4 to 9
move 4 from 7 to 8
move 4 from 4 to 3
move 1 from 7 to 9
move 4 from 8 to 6
move 1 from 3 to 4
move 1 from 3 to 5
move 2 from 4 to 7
move 4 from 6 to 3
move 2 from 9 to 1
move 2 from 7 to 4
move 1 from 5 to 1
move 1 from 3 to 4
move 1 from 9 to 3
move 4 from 4 to 5
move 2 from 5 to 3
move 1 from 5 to 7
move 1 from 5 to 8
move 2 from 6 to 4
move 3 from 1 to 3
move 21 from 3 to 5
move 3 from 6 to 1
move 1 from 7 to 1
move 4 from 2 to 6
move 1 from 8 to 2
move 10 from 2 to 4
move 4 from 1 to 2
move 1 from 6 to 5
move 2 from 6 to 9
move 7 from 4 to 9
move 1 from 6 to 5
move 3 from 9 to 4
move 6 from 2 to 8
move 3 from 9 to 1
move 8 from 4 to 3
move 1 from 9 to 4
move 21 from 5 to 7
move 1 from 1 to 3
move 2 from 9 to 6
move 14 from 7 to 1
move 2 from 4 to 1
move 2 from 8 to 7
move 1 from 8 to 2
move 11 from 2 to 9
move 8 from 9 to 6
move 4 from 7 to 1
move 1 from 7 to 4
move 2 from 3 to 5
move 1 from 1 to 6
move 1 from 8 to 2
move 3 from 7 to 5
move 6 from 1 to 7
move 1 from 8 to 7
move 1 from 4 to 5
move 4 from 6 to 5
move 6 from 7 to 6
move 3 from 9 to 1
move 1 from 7 to 3
move 11 from 5 to 1
move 1 from 5 to 2
move 9 from 6 to 4
move 1 from 7 to 3
move 2 from 6 to 1
move 1 from 2 to 1
move 1 from 2 to 6
move 14 from 1 to 5
move 1 from 8 to 4
move 10 from 1 to 5
move 3 from 5 to 1
move 8 from 3 to 8
move 16 from 5 to 7
move 2 from 1 to 9
move 3 from 8 to 1
move 1 from 2 to 4
move 6 from 7 to 4
move 3 from 5 to 8
move 2 from 3 to 6
move 7 from 1 to 7
move 14 from 4 to 3
move 9 from 7 to 8
move 2 from 4 to 1
move 9 from 8 to 4
move 7 from 8 to 2
move 6 from 1 to 8
move 1 from 9 to 7
move 1 from 1 to 6
move 1 from 9 to 6
move 1 from 5 to 9
move 1 from 5 to 3
move 9 from 4 to 9
move 3 from 3 to 6
move 8 from 6 to 3
move 1 from 2 to 9
move 8 from 9 to 8
move 6 from 2 to 9
move 2 from 6 to 1
move 7 from 8 to 6
move 2 from 9 to 6
move 8 from 7 to 8
move 1 from 4 to 5
move 9 from 3 to 5
move 2 from 1 to 4
move 1 from 7 to 4
move 2 from 4 to 3
move 11 from 8 to 1
move 1 from 4 to 7
move 1 from 7 to 8
move 5 from 1 to 3
move 4 from 6 to 4
move 2 from 4 to 8
move 1 from 4 to 8
move 7 from 8 to 9
move 1 from 8 to 9
move 1 from 8 to 5
move 18 from 3 to 2
move 17 from 2 to 7
move 6 from 5 to 4
move 1 from 2 to 5
move 4 from 4 to 6
move 4 from 6 to 9
move 15 from 7 to 9
move 2 from 1 to 6
move 2 from 7 to 9
move 28 from 9 to 2
move 1 from 6 to 7
move 4 from 6 to 9
move 3 from 1 to 7
move 2 from 6 to 3
move 1 from 4 to 7
move 8 from 9 to 5
move 13 from 5 to 3
move 1 from 5 to 7
move 3 from 9 to 4
move 8 from 3 to 7
move 28 from 2 to 5
move 1 from 9 to 8
move 4 from 3 to 4
move 4 from 7 to 5
move 2 from 3 to 9
move 21 from 5 to 4
move 1 from 5 to 7
move 1 from 3 to 5
move 3 from 5 to 7
move 1 from 1 to 3
move 3 from 7 to 3
move 5 from 7 to 6
move 10 from 4 to 8
move 6 from 5 to 4
move 1 from 9 to 3
move 15 from 4 to 5
move 10 from 4 to 7
move 3 from 3 to 7
move 1 from 3 to 4
move 1 from 3 to 4
move 7 from 5 to 1
move 2 from 4 to 7
move 1 from 9 to 2
move 2 from 6 to 9
move 1 from 5 to 3
move 1 from 3 to 8
move 10 from 7 to 9
move 2 from 8 to 1
move 9 from 9 to 2
move 1 from 4 to 3
move 9 from 8 to 7
move 1 from 2 to 8
move 5 from 5 to 4
move 1 from 3 to 2
move 5 from 4 to 3
move 3 from 5 to 9
move 6 from 7 to 3
move 1 from 6 to 5
move 5 from 9 to 7
move 2 from 5 to 6
move 3 from 6 to 7
move 4 from 1 to 4
move 6 from 2 to 7
move 17 from 7 to 5
move 1 from 6 to 1
move 5 from 3 to 6
move 10 from 7 to 2
move 1 from 8 to 4
move 1 from 9 to 8
move 3 from 4 to 1
move 1 from 7 to 4
move 5 from 5 to 9
move 2 from 8 to 7
move 3 from 3 to 7
move 4 from 2 to 3
move 3 from 4 to 6
move 7 from 5 to 8
move 7 from 2 to 8
move 4 from 9 to 8
move 12 from 8 to 3
move 17 from 3 to 2
move 1 from 7 to 9
move 1 from 3 to 9
move 3 from 9 to 1
move 2 from 5 to 1
move 1 from 3 to 5
move 4 from 5 to 8
move 6 from 8 to 1
move 17 from 2 to 3
move 13 from 3 to 2
move 1 from 3 to 9
move 1 from 8 to 4
move 1 from 4 to 8
move 1 from 9 to 1
move 2 from 7 to 2
move 8 from 6 to 2
move 2 from 7 to 5
move 9 from 1 to 3
move 13 from 2 to 9
move 6 from 1 to 4
move 6 from 4 to 5
move 3 from 8 to 1
move 2 from 1 to 8
move 8 from 5 to 7
move 2 from 3 to 1
move 9 from 3 to 1
move 3 from 8 to 2
move 1 from 1 to 9
move 1 from 3 to 9
move 6 from 7 to 3
move 4 from 2 to 7
move 14 from 1 to 6
move 2 from 3 to 9
move 3 from 3 to 7
move 6 from 2 to 1
move 2 from 1 to 2
move 9 from 6 to 3
move 11 from 9 to 5
move 9 from 7 to 6
move 6 from 6 to 2
move 1 from 1 to 8
move 5 from 9 to 4
move 1 from 8 to 5
move 9 from 2 to 7
move 10 from 5 to 8
"#;
