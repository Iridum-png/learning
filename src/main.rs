extern crate rand;
use rand::Rng;
use std::collections::BTreeMap;

fn main() {
    // Call function 1
    _call_one();
    print!("\n");

    // Call function 2
    _call_two();
    print!("\n");

    // Call function 3
    _call_three();
    print!("\n");

    // Call function 4
    _call_four();
    print!("\n");

    // Call function 5
    _call_five();
    print!("\n");

    // Call function 6
    _call_six();
    print!("\n");

    // Call function 7
    _call_seven();
    print!("\n");

    // Call function 8
    _call_eight();
    print!("\n");
}

fn _call_one() {
    println!("{}", _one(2, 5, 8));
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
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert("a".to_string(), "01".to_string());
    map.insert("b".to_string(), "02".to_string());
    map.insert("c".to_string(), "03".to_string());
    map.insert("d".to_string(), "04".to_string());
    map.insert("e".to_string(), "05".to_string());
    map.insert("f".to_string(), "06".to_string());
    map.insert("g".to_string(), "07".to_string());
    map.insert("h".to_string(), "08".to_string());
    map.insert("i".to_string(), "09".to_string());
    map.insert("j".to_string(), "10".to_string());
    map.insert("k".to_string(), "11".to_string());
    map.insert("l".to_string(), "12".to_string());
    map.insert("m".to_string(), "13".to_string());
    map.insert("n".to_string(), "14".to_string());
    map.insert("o".to_string(), "15".to_string());
    map.insert("p".to_string(), "16".to_string());
    map.insert("q".to_string(), "17".to_string());
    map.insert("r".to_string(), "18".to_string());
    map.insert("s".to_string(), "19".to_string());
    map.insert("t".to_string(), "20".to_string());
    map.insert("u".to_string(), "21".to_string());
    map.insert("v".to_string(), "22".to_string());
    map.insert("w".to_string(), "23".to_string());
    map.insert("x".to_string(), "24".to_string());
    map.insert("y".to_string(), "25".to_string());
    map.insert("z".to_string(), "26".to_string());
    map.insert("A".to_string(), "27".to_string());
    map.insert("B".to_string(), "28".to_string());
    map.insert("C".to_string(), "29".to_string());
    map.insert("D".to_string(), "30".to_string());
    map.insert("E".to_string(), "31".to_string());
    map.insert("F".to_string(), "32".to_string());
    map.insert("G".to_string(), "33".to_string());
    map.insert("H".to_string(), "34".to_string());
    map.insert("I".to_string(), "35".to_string());
    map.insert("J".to_string(), "36".to_string());
    map.insert("K".to_string(), "37".to_string());
    map.insert("L".to_string(), "38".to_string());
    map.insert("M".to_string(), "39".to_string());
    map.insert("N".to_string(), "40".to_string());
    map.insert("O".to_string(), "41".to_string());
    map.insert("P".to_string(), "42".to_string());
    map.insert("Q".to_string(), "43".to_string());
    map.insert("R".to_string(), "44".to_string());
    map.insert("S".to_string(), "45".to_string());
    map.insert("T".to_string(), "46".to_string());
    map.insert("U".to_string(), "47".to_string());
    map.insert("V".to_string(), "48".to_string());
    map.insert("W".to_string(), "49".to_string());
    map.insert("X".to_string(), "50".to_string());
    map.insert("Y".to_string(), "51".to_string());
    map.insert("Z".to_string(), "52".to_string());
    map.insert("0".to_string(), "53".to_string());
    map.insert("1".to_string(), "54".to_string());
    map.insert("2".to_string(), "55".to_string());
    map.insert("3".to_string(), "56".to_string());
    map.insert("4".to_string(), "57".to_string());
    map.insert("5".to_string(), "58".to_string());
    map.insert("6".to_string(), "59".to_string());
    map.insert("7".to_string(), "60".to_string());
    map.insert("8".to_string(), "61".to_string());
    map.insert("9".to_string(), "62".to_string());
    for elem in _three(&map).iter() {
        println!("{}", elem);
    }
}

fn _three<'a>(key_value: &'a BTreeMap<String, String>) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for pair in key_value.iter() {
        result.push(pair.0);
    }
    return result;
}

fn _call_four() {
    let mut random = rand::thread_rng();
    let mut arr = Vec::<i32>::new();
    for _ in 0..random.gen_range(1..15) {
        arr.push(random.gen_range(1..100));
    }
    let result = _four(arr);
    println!("{} {}", result.0, result.1);
}

fn _four(mut arr: Vec<i32>) -> (i32, i32) {
    arr.sort();
    return (arr[arr.len() - 1], arr[arr.len() - 2]);
}

fn _call_five() {
    println!("{}", _five("qWeRtYuIoP"));
}

fn _five(string: &str) -> String {
    let mut swapped = String::new();
    for character in string.chars() {
        if character.is_uppercase() {
            swapped += &character.to_string().to_ascii_lowercase();
        } else {
            swapped += &character.to_string().to_ascii_uppercase();
        }
    }
    return swapped;
}

fn _call_six() {
    println!("{}", _six(10));
}

fn _six(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..n + 1 {
        total += i ^ 3;
    }
    return total;
}

fn _call_seven() {
    let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for elem in _seven(&list, 3).iter() {
        print!("{}", elem);
    }
    print!("\n");
}

fn _seven(list: &[i32], n: i32) -> Vec<i32> {
    let mut total = Vec::<i32>::new();
    for i in 0..list.len() {
        total.push(list[i] / n);
    }
    return total;
}

fn _call_eight() {
    let num1 = 4;
    let num2 = 8;
    for elem in _eight(num1, num2) {
        println!("{} | {}", elem.0, elem.1)
    }
}

fn _eight(num1: i32, num2: i32) -> Vec<(i32, i32)> {
    let mut nums = Vec::<(i32, i32)>::new();
    for i in num1..num2 + 1 {
        for j in num1..num2 + 1 {
            let mut edit = (i, j);
            if i > j {
                edit = (j, i);
            }
            if !nums.contains(&edit) {
                nums.push(edit);
            }
        }
    }
    let sorted = _sort(&mut nums);
    return sorted.to_vec();
}

fn _sort(arr: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j + 1].0 < arr[j].0 {
                arr.swap(j, j + 1);
            }
        }
    }
    return arr.to_vec();
}
