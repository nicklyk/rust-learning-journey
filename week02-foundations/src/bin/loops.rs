fn main() {
    loop {
        println!("again!");
        break; // used to cut off infinite loop
    }

    break_example();
}

fn break_example() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}