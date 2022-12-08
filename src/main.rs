use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::usize;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let top_n_value = &args[1];

    let file = File::open("data.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents.is_empty(), false);

    let mut total_data = get_elf_calories(contents);
    total_data.sort();
    total_data.reverse();
    
    let top_most_calories = get_top_n_most_calories(total_data, top_n_value.parse::<usize>().unwrap());

    println!("{top_most_calories}");
    Ok(())
}

fn get_elf_calories(contents: String) -> Vec<i32> {
    let elf_calories = contents.split("\n\n");
    let mut total_data = Vec::new();

    for data in elf_calories {
        let calories = data.split("\n");
        let mut total_calories = 0;
        for values in calories {
            let value = values.parse::<i32>().unwrap();
            total_calories += value;
        }
        total_data.push(total_calories);
    }

    return total_data
}

fn get_top_n_most_calories(total_data: Vec<i32>, n: usize) -> i32 {
    let mut i = 0;
    let mut top_n_calories = 0;
    while i < n {
        top_n_calories += total_data[i];
        i += 1;
    }

    return top_n_calories;
}