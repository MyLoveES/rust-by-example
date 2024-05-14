fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a v: {:?}", val),
    }

    match *reference {
        val => println!("Got a v: {:?}", val),
    }

    let _not_a_refrence = 3;

    let ref _is_a_refrence = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a ref: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("Add 10 , mut_value : {:?}", m);
        }
    }
}
