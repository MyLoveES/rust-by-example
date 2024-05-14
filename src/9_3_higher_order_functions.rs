fn is_odd(n : u32) -> bool {
    n % 2 == 1
} 

fn main() {
    let upper = 1000;

    let sum_of_squared_odd_numbers: u32 = 
            (0..).map(|n| n * n)
                 .take_while(|&n_squared| n_squared < upper)
                 .filter(|&n_squared| is_odd(n_squared))
                 .sum();

    println!("functional: {}", sum_of_squared_odd_numbers);
}
