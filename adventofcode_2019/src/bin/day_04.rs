/* 
https://adventofcode.com/2019/day/4
john shiles 
*/

use itertools::Itertools;

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn valid_password(password: u32) -> bool {
    /*
    Check if there is at least two consecutive numbers AND each number is 
    greater than or equal to the prior number.
    */
    let mut consecutive_found: bool = false;
    let password_vec = number_to_vec(password);

    for (i, j) in (0..password_vec.len()).tuple_windows() {
        if  &password_vec[i as usize] == &password_vec[j as usize] {
            consecutive_found = true;
        }
        if &password_vec[i as usize] > &password_vec[j as usize] {
            return false
        }
    }
    return consecutive_found
}

fn valid_password_extended(password: u32) -> bool {
    /*
    If valid_password is true, then apply an additional rule: check if there 
    is at least 1 pattern of exactly 2 consecutive numbers.
    */
    let mut valid: bool = false;
    if valid_password(password) {
        let password_vec = number_to_vec(password);
        for (i, j, k, l) in (0..password_vec.len()).tuple_windows::<(_, _, _, _)>() {
            if i == 0 && &password_vec[i as usize] == &password_vec[j as usize] && 
                &password_vec[i as usize] != &password_vec[k as usize] {
                // beginning of the list
                valid = true;
            }
            else if l == password_vec.len()-1 && 
                &password_vec[k as usize] == &password_vec[l as usize] && 
                &password_vec[j as usize] != &password_vec[k as usize] {
                // ending of the list
                valid = true;
            } else if &password_vec[i as usize] != &password_vec[j as usize] && 
                &password_vec[j as usize] == &password_vec[k as usize] &&
                &password_vec[k as usize] != &password_vec[l as usize] { 
                    valid = true;
            }
        }
    }
    return valid
}

fn main() {
    /*
    part 1: How many valid passwords are there within the range 123257-647015
    according the base logic?
    */
    let mut count = 0;
    for i in 123257..647016 {
        if valid_password(i) {
            count = count + 1;
        }
    }
    println!("Valid codes (part 1): {:?}", count); // 2220

    /*
    part 2: How many valid passwords are there within the range 123257-647015 
    according to the extended logic?
    */
    count = 0;
    for i in 123257..647016 {
        if valid_password_extended(i) {
            count = count + 1;
        }
    }
    println!("Valid codes (part 2): {:?}", count); // 1515
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password () {
        assert_eq!(true, valid_password(111111));
        assert_eq!(false, valid_password(223450));
        assert_eq!(false, valid_password(123789));
    }

    #[test]
    fn test_valid_password_extended () {
        assert_eq!(false, valid_password_extended(111111));
        assert_eq!(true, valid_password_extended(112233));
        assert_eq!(false, valid_password_extended(123444));
        assert_eq!(true, valid_password_extended(111122));
    }
}