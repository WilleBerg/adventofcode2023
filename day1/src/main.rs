fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    // println!("Part A {}", run_a(input.clone()));
    println!("Part B {}", run_b(input.clone()));
}

fn run_b(input: Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let number_strings: Vec<String> = create_vec();
    let number_map: std::collections::HashMap<String, char> = create_map();

    for line in &input {
        let mut val: String = String::from("");
        let mut curr_string: String = "".to_string();
        for ch in line.chars() {
            // println!("{}", curr_string);
            match ch.to_digit(10) {
                Some(_) => { val.push(ch); break },
                None => {
                    curr_string.push(ch);
                    let mut tmp_bool: bool = false;
                    for num in &number_strings {
                        if curr_string.contains(num) {
                            val.push(number_map.get(num).unwrap().clone());
                            tmp_bool = true;
                            break;
                        }
                    }
                    if tmp_bool { break; }
                    continue;
                },
            }
        }
        // println!("Done once: {}", val);
        let mut curr_string: String = "".to_string();
        for ch in line.chars().rev() {
            // println!("{}", curr_string);
            match ch.to_digit(10) {
                Some(_) => { val.push(ch); break },
                None => {
                    curr_string.push(ch);
                    let tmp: String = curr_string.chars().rev().collect();
                    let mut tmp_bool = false;
                    for num in &number_strings {
                        if tmp.contains(num) {
                            val.push(number_map.get(num).unwrap().clone());
                            tmp_bool = true;
                            break;
                        }
                    }
                    if tmp_bool { break; }
                    continue;
                },
            }
        }
        // println!("Done twice: {}", val);
        result += val.parse::<i32>().unwrap();
        // println!("{}", result);
    }

    result
}

fn create_vec() -> Vec<String> {
    let number_strings: Vec<String> = vec![
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
        ]
        .into_iter()
        .map(String::from)
        .collect();
    number_strings
}

fn create_map() -> std::collections::HashMap<String, char> {
    let mut map = std::collections::HashMap::new();
    
    map.insert("zero".to_string(), '0');
    map.insert("one".to_string(), '1');
    map.insert("two".to_string(), '2');
    map.insert("three".to_string(), '3');
    map.insert("four".to_string(), '4');
    map.insert("five".to_string(), '5');
    map.insert("six".to_string(), '6');
    map.insert("seven".to_string(), '7');
    map.insert("eight".to_string(), '8');
    map.insert("nine".to_string(), '9');

    map
}

fn run_a(input: Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut val: String = String::from("");
    for line in &input {
        for ch in line.chars() {
            match ch.to_digit(10) {
                Some(_) => { val.push(ch); break },
                None => continue,
            }
        }
        for ch in line.chars().rev() {
            match ch.to_digit(10) {
                Some(_) => { val.push(ch); break },
                None => continue,
            }
        }
        result += val.parse::<i32>().unwrap();
        println!("{}", result);
        val = String::from("");
    }
    result
}

