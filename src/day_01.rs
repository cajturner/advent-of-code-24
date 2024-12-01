

fn distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut total_distance = 0;
    for i in 0..left.len() {
        total_distance += (left[i] - right[i]).abs();
    }
    println!("{}", total_distance);
    return total_distance;
}

fn part2(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut right_counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    
    for &num in right.iter() {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let mut total_distance = 0;
    for &num in left.iter() {
        let count = right_counts.get(&num).unwrap_or(&0);
        total_distance += &num * count;
    }
    println!("{}", total_distance);
    return total_distance;
}


pub fn main() {
    let input = include_str!("day_01/input.txt");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
                left.push(num1);
                right.push(num2);
            }
        }
    }
    left.sort();
    right.sort();
    distance(left.clone(), right.clone());
    part2(left.clone(), right.clone());
}
