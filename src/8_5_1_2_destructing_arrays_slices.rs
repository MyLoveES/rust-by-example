fn main() {
    let array = [9, -2, 6];

    match array {
        [0, second, third] => 
            println!("0 = 0, 1 = {}, 2 = {}", second, third),
        [1, _, third] => println!(
            "0 = 1, 2 = {}, 1 was ignored",
            third
        ),
        [-1, second, ..] => println!(
            "0 = -1, 1 = {}, other ignored",
            second
        ),
        [3, second, tail @ ..] => println!(
            "0 = 3, 1 = {}, other {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "0 = {}, middle = {:?}, 2 = {}",
            first, middle, last
        ),
    }
}
