use std::collections::HashMap;
use std::fs;

fn main() -> std::io::Result<()> {
    let input_path = "input/input.txt";
    let content = fs::read_to_string(input_path)?;

    let (mut left_list, mut right_list): (Vec<i32>, Vec<i32>) = content
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    let frequency_map: HashMap<i32, usize> =
        right_list.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| num * (*frequency_map.get(&num).unwrap_or(&0) as i32))
        .sum();

    println!("Total distance: {}", total_distance);
    println!("Similarity score: {}", similarity_score);

    Ok(())
}

