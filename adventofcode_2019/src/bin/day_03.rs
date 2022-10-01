/* 
https://adventofcode.com/2019/day/3
john shiles 
*/

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use array_tool::vec::Intersect;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

pub trait Add {
    fn add(&self, instr: &str) -> Result<Vec<Point>, Error>;
}

pub trait Manh {
    fn manh(&self) -> i32;
}

impl Add for Point {
    fn add(&self, instr: &str) -> Result<Vec<Point>, Error> {
        let mut path: Vec<Point> = vec![];
        let mut chars = instr.chars();
        let direction = chars.next();
        let magnitude: i32 = chars.as_str().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        for i in 1..magnitude+1 {
            let new_point: Point = match direction {
                Some('R') => Point { x: self.x+i, y: self.y, },
                Some('L') => Point { x: self.x-i, y: self.y, },
                Some('U') => Point { x: self.x, y: self.y+i, },
                Some('D') => Point { x: self.x, y: self.y-i, },
                _other => panic!("Error obtaining direction {:?}", direction)
            };
            path.push(new_point);
        }   
        Ok(path)
    }
}

impl Manh for Point {
    fn manh(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", &self.x, &self.y)
    }
}

fn read_input(path: &str) -> Result<(String, String), Error> {   
    let file = File::open(path)?;
    let mut br = BufReader::new(file);

    let mut path1 = String::new();
    br.read_line(&mut path1).expect("read_line error");

    let mut path2 = String::new();
    br.read_line(&mut path2).expect("read_line error");

    Ok((path1, path2))
}

fn construct_path(input: &str) -> Vec<Point> {
    let start: Point = Point { x: 0, y: 0, };
    let mut path: Vec<Point> = vec![start];

    let parts: Vec<&str> = input.trim().split(",").collect();
    for part in parts {
        let mut additional_path = match path.last().unwrap().add(&part) {
            Ok(p) => p,
            Err(e) =>  panic!("Error obtaining project root {:?}", e),
        };
        // println!("{}", additional_path.last().unwrap());
        path.append(&mut additional_path);
    }

    path
}

fn find_closest_intersection_distance(input1: &str, input2: &str) -> i32 {
    let path_1: Vec<Point> = construct_path(input1);
    let path_2: Vec<Point> = construct_path(input2);

    let mut lowest = 0;
    for p in path_1.intersect(path_2) {
        // println!("Intersection point: {}, Man Distance: {:?}", p, p.manh());
        if lowest == 0 || p.manh() < lowest {
            lowest = p.manh();
        }
    }
    println!("Lowest intersection: {:?}", lowest);
    lowest
}

fn find_closest_intersection_steps(input1: &str, input2: &str) -> i32 {
    let path_1: Vec<Point> = construct_path(input1);
    let path_2: Vec<Point> = construct_path(input2);

    let mut lowest = 0;
    for p in path_1.intersect(path_2.clone()) {
        let index_1 = path_1.iter().position(|&r| r == p).unwrap();
        let index_2 = path_2.iter().position(|&r| r == p).unwrap();
        if lowest == 0 || index_1+index_2 < lowest {
            lowest = index_1+index_2;
        }
    }
    println!("Lowest steps to intersection: {:?}", lowest);
    lowest as i32
}



fn main () {
    let project_root_dir = match project_root::get_project_root() {
        Ok(p) => p,
        Err(e) =>  panic!("Error obtaining project root {:?}", e)
    };
    let input_data_file = format!("{}{}", 
        project_root_dir.into_os_string().into_string().unwrap(), 
        "/data/day_03.txt");

    /*
    Part 1: find closest intersection by manhatten distance.
    */
    let (input_1, input_2) = read_input(&input_data_file).unwrap();
    let p1_answer = find_closest_intersection_distance(&input_1, &input_2);
    println!("{:?}", p1_answer);

    /*
    Part 2: find closest intersection by sum of steps.
    */
    let (input_1_p2, input_2_p2) = read_input(&input_data_file).unwrap();
    let p2_answer = find_closest_intersection_steps(&input_1_p2, &input_2_p2);
    println!("{:?}", p2_answer);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_intersection_distance () {
        let test_input_1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let test_input_2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(
            find_closest_intersection_distance(&test_input_1, &test_input_2),
            159);
    }
}
