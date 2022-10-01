/* 
https://adventofcode.com/2019/day/1
john shiles 
*/

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn read_input(path: &str) -> Result<Vec<i32>, Error> {
    let mut v = Vec::new();
    
    let file = File::open(path)?;
    let br = BufReader::new(file);
    for line in br.lines() {
        let line = line?;
        let n = line   
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; 
        v.push(n);
    }
    Ok(v)
}

fn fuel_requirements(mass: i32) -> i32 {
    /*
    Fuel required to launch a given module is based on its mass. Specifically,
    to find the fuel required for a module, take its mass, divide by three,
    round down, and subtract 2.
    */
    (mass / 3) - 2
}

fn fuel_requirements_advanced(mass: i32) -> i32 {
    /*
    fuel requirements, plus account for added fuel weight.
    */    
    let mut fuel_mass = fuel_requirements(mass);
    let mut total_fuel = fuel_mass;
    loop {
        fuel_mass = fuel_requirements(fuel_mass);
        if fuel_mass < 1 {
            break;
        }
        total_fuel += fuel_mass;
    }
    return if total_fuel >= 0 { total_fuel } else { 0 }
}

fn main() {
    let project_root_dir = match project_root::get_project_root() {
        Ok(p) => p,
        Err(e) =>  panic!("Error obtaining project root {:?}", e)
    };
    let input_data_file = format!("{}{}", 
        project_root_dir.into_os_string().into_string().unwrap(), 
        "/data/day_01.txt");

    let masses_result = read_input(&input_data_file);
    let masses: Vec<i32> = match masses_result {
        Ok(m) => m,
        Err(error) => panic!("Problem parsing the masses: {:?}", error),
    };

    // Part 1
    let fuel_requirement: i32 = masses.clone()
        .into_iter()
        .map(|n| fuel_requirements(n))
        .sum();
    println!("{:?}", &fuel_requirement); // 3363929


    // Part 2 - account for the added weight by fuel.
   let fuel_requirement_p2: i32 = masses.clone()
        .into_iter()
        .map(|n| fuel_requirements_advanced(n))
        .sum();
    println!("{:?}", &fuel_requirement_p2); // 5043026
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_requirements_basic () {
        assert_eq!(654, fuel_requirements(1969));
        assert_eq!(33583, fuel_requirements(100756));
    }

    #[test]
    fn test_fuel_requirements_advanced () {
        assert_eq!(966, fuel_requirements_advanced(1969));
        assert_eq!(50346, fuel_requirements_advanced(100756));
    }
}