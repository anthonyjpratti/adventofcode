use std::error::Error;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "2019-1", about = "First day of Advent of Code")]
struct Opt {
    #[structopt(short = "p", long = "path")]
    path: String,
}

fn main() {
    let opt = Opt::from_args();
    let input = read_file(opt.path).unwrap();
    let output = determine_fuel_requirements(input);
    println!("Solution: {}", output);
}

fn read_file(path: String) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut res = Vec::new();
    for line in fs::read_to_string(path)?.split('\n') {
        if line != "" {
            let entry = line.parse::<u32>()?;
            res.push(entry);
        }
    }
    Ok(res)
}

fn determine_fuel_requirements(modules: Vec<u32>) -> u32 {
    modules.iter().map(calc_fuel_per_module).sum()
}

fn calc_fuel_per_module(module: &u32) -> u32 {
    let mut total = 0;
    let mut curr = *module;
    while curr > 5 {
        let temp = curr / 3 - 2;
        total += temp;
        curr = temp;
    }
    total
}
