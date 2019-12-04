use std::error::Error;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "2019-2", about = "Second day of Advent of Code")]
struct Opt {
    #[structopt(short = "p", long = "path")]
    path: String,
}

fn main() {
    let opt = Opt::from_args();
    let mut input = read_file(opt.path).unwrap();
    input[1] = 12;
    input[2] = 2;
    let mut first = input.clone();
    compute(&mut first);
    println!("First Solution: {}", first[0]);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vec = input.clone();
            vec[1] = noun;
            vec[2] = verb;
            compute(&mut vec);
            if vec[0] == 19690720 {
                println!("Second Solution: {}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn read_file(path: String) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut res = Vec::new();
    for line in fs::read_to_string(path)?.split(',') {
        if line != "" {
            let entry = line.parse::<usize>()?;
            res.push(entry);
        }
    }
    Ok(res)
}

fn apply_opcode(ops: &mut Vec<usize>, p: usize, fun: &dyn Fn(usize, usize) -> usize) {
    let c1 = ops[p + 1];
    let c2 = ops[p + 2];
    let c3 = ops[p + 3];
    ops[c3] = fun(ops[c1], ops[c2]);
}

fn add(x: usize, y: usize) -> usize {
    x + y
}

fn multi(x: usize, y: usize) -> usize {
    x * y
}

fn compute(ops: &mut Vec<usize>) {
    let mut p: usize = 0;
    loop {
        let c = ops[p];
        match c {
            1 => apply_opcode(ops, p, &add),
            2 => apply_opcode(ops, p, &multi),
            99 => break,
            _ => break,
        }
        p += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_1() {
        let mut opcodes = vec![1, 0, 0, 0, 99];
        compute(&mut opcodes);
        assert_eq!(opcodes, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_compute_2() {
        let mut opcodes = vec![2, 3, 0, 3, 99];
        compute(&mut opcodes);
        assert_eq!(opcodes, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_compute_3() {
        let mut opcodes = vec![2, 4, 4, 5, 99, 0];
        compute(&mut opcodes);
        assert_eq!(opcodes, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_compute_4() {
        let mut opcodes = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        compute(&mut opcodes);
        assert_eq!(opcodes, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
