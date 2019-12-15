use std::error::Error;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "2019-4", about = "Fourth day of Advent of Code")]
struct Opt {
    #[structopt(short = "p", long = "path")]
    path: String,
}

fn main() {
    let opt = Opt::from_args();
    let input = read_file(opt.path).unwrap();
    let output = count_possiblities(input.0, input.1);
    println!("First Solution: {}", output)
}

fn read_file(path: String) -> Result<(usize, usize), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let mut parts = contents.split('-');
    let start = parts.next().unwrap().parse::<usize>()?;
    let stop = parts.next().unwrap().parse::<usize>()?;
    Ok((start, stop))
}

fn count_possiblities(start: usize, stop: usize) -> usize {
    let mut count = 0;
    for x in start..=stop {
        if check(x) {
            count += 1;
        }
    }
    count
}

const MASK: usize = 100000;

fn check(val: usize) -> bool {
    check_not_decrease(val) && check_two_adjacent(val)
}

fn check_not_decrease(val: usize) -> bool {
    let mut mask = MASK;
    let mut max = 0;
    while mask > 0 {
        let curr = val / mask % 10;
        if curr < max {
            return false;
        }
        max = curr;
        mask = mask / 10;
    }
    return true;
}

fn check_two_adjacent(val: usize) -> bool {
    let mut mask = MASK;
    let mut last = 0;
    let mut count = 0;
    while mask > 0 {
        let curr = val / mask % 10;
        if curr == last {
            count += 1;
        } else if count == 1 {
            return true;
        } else {
            count = 0
        }
        last = curr;
        mask = mask / 10;
    }
    count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_two_adjacent() {
        assert_eq!(check_two_adjacent(123456), false);
        assert_eq!(check_two_adjacent(123356), true);
        assert_eq!(check_two_adjacent(112356), true);
        assert_eq!(check_two_adjacent(122456), true);
        assert_eq!(check_two_adjacent(123444), false);
        assert_eq!(check_two_adjacent(111144), true);
    }

    #[test]
    fn test_check_not_decrease() {
        assert_eq!(check_not_decrease(123456), true);
        assert_eq!(check_not_decrease(111111), true);
        assert_eq!(check_not_decrease(123443), false);
        assert_eq!(check_not_decrease(123432), false);
    }

}
