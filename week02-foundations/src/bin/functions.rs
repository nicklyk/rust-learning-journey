fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(2, 'a');
}

fn another_function() {
    println!("Another function.");
    yet_another_function(5i32);
}

fn yet_another_function(x: i32) {
    println!("Yet another function and x = {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}