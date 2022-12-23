/*
Read file with include_str of a followng structure
 
Lines contain numbers or are empty
Find a sum of every serial group of lines with numbers
Find the biggest sum of all serial groups
*/

fn main() {
    let mut max_sum = 0;
    let mut sum = 0;
    for line in include_str!("input.txt").lines() {
        if let Ok(num) = line.parse::<i32>() {
            sum += num;
        } else {
            if sum > max_sum {
                max_sum = sum;
            }
            sum = 0;
        }
    }
    println!("Max sum: {}", max_sum);
}