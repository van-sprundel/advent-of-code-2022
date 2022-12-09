use anyhow::Result;

fn main() -> Result<()> {
    println!("{}", part_1()?);
    println!("{}", part_2()?);
    Ok(())
}

fn part_1() -> Result<i32> {
    let mut res = 0;
    let array = EXAMPLE
        .lines()
        .map(|line: &str| {
            line.chars()
                .map(|c| c.to_digit(10).expect("can't find digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let y_len = array.len();
    for y in 0..y_len {
        let row = &array[y];
        let x_len = row.len();
        for x in 0..row.len() {
            let num = row[x];
            let mut visible = true;
        }
    }
    Ok(0)
}

fn part_2() -> Result<i32> {
    Ok(0)
}

const EXAMPLE: &str = r#"30373
25512
65332
33549
35390
"#;

const INPUT: &str = r#""#;
