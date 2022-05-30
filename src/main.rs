extern crate rand;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Call function 2
    // _call_two();
    
    // Call function 3
    _call_three();

    // Call function 6
    // _call_six();

    // Call function 7
    // _call_seven();
}

fn _one(num1: i16, num2: i16, num3: i16) -> i16 {
    if num1 % 2 == 0 {
        return num1 * num2;
    } else {
        return num1 * num3;
    }
}

fn _call_two() {
    let deck = (1..53).collect::<Vec<u32>>();
    _two(deck);
}

fn _two(deck: Vec<u32>) {
    let mut random = rand::thread_rng();

    let choice = deck[random.gen_range(0..52)];
    println!("{}", choice)
}

fn _call_three() {
    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
        ("A", 27),
        ("B", 28),
        ("C", 29),
        ("D", 30),
        ("E", 31),
        ("F", 32),
        ("G", 33),
        ("H", 34),
        ("I", 35),
        ("J", 36),
        ("K", 37),
        ("L", 38),
        ("M", 39),
        ("N", 40),
        ("O", 41),
        ("P", 42),
        ("Q", 43),
        ("R", 44),
        ("S", 45),
        ("T", 46),
        ("U", 47),
        ("V", 48),
        ("W", 49),
        ("X", 50),
        ("Y", 51),
        ("Z", 52),
        ("0", 53),
        ("1", 54),
        ("2", 55),
        ("3", 56),
        ("4", 57),
        ("5", 58),
        ("6", 59),
        ("7", 60),
        ("8", 61),
        ("9", 62),
    ]);
    for elem in _three(map).iter() {
        println!("{}", elem);
    }
}

fn _three<'a>(key_value: HashMap<&'a str, i32>) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for pair in key_value.iter() {
        result.push(pair.0);
    }
    return result;
}

fn _call_six() {
    println!("{}", _six(10));
}

fn _six(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..n+1 {
        total += i^3;
    }
    return total;
}

fn _call_seven() {
    let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for elem in _seven(&list, 3).iter() {
        print!("{}", elem);
    }
}
fn _seven(list: &[i32], n: i32) -> Vec<i32> {
    let mut total = Vec::<i32>::new();
    for i in 0..list.len() {
        total.push(list[i] / n);
    }
    return total;
}

// def eight(num1, num2):
//     nums = []
//     for i in range(num1, num2+1):
//         for j in range(num1, num2+1):
//             edit = sorted((i, j))
//             nums.append(tuple(edit))
//     return sorted(list(set([i for i in nums])))
