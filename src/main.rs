extern crate rand;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Call function 2
    let deck = (1..53).collect::<Vec<u32>>();
    _two(deck);

    // Call function 3
}

fn _one(num1: i16, num2: i16, num3: i16) -> i16 {
    if num1 % 2 == 0 {
        return num1 * num2;
    } else {
        return num1 * num3;
    }
}

fn _two(deck: Vec<u32>) {
    let mut random = rand::thread_rng();

    let choice = deck[random.gen_range(0..52)];
    println!("{}", choice)
}

fn three<'a>(key_value: HashMap<&'a str, &str>) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for pair in key_value.iter() {
        result.push(pair.0);
    }
    return result;
}

// def three(array):
//     return list(dict.fromkeys(array))

// def four(array):
//     array = sorted(array)
//     return array[-1], array[-2]

// def five(string):
//     return string.swapcase()

// def six(n):
//     total = 0
//     for i in range(1, n + 1):
//         total += i**3
//     return total

// def seven(list, num):
//     list2 = []
//     for item in list:
//         list2.append(item // num)
//     return list2

// def eight(num1, num2):
//     nums = []
//     for i in range(num1, num2+1):
//         for j in range(num1, num2+1):
//             edit = sorted((i, j))
//             nums.append(tuple(edit))
//     return sorted(list(set([i for i in nums])))
