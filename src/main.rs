fn main() {
    let start = 1;
    let end = 10;
    let sum = sum_of_evens(start, end);
    println!("The sum of even numbers from {} to {} is {}", start, end, sum);
}

fn sum_of_evens(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    for number in start..=end {
        if is_even(number) {
            sum += number;
        }
    }
    sum
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

