fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}


fn other_fun() {
    println!("Hello, world!");

    another_function(5);
    measurement(32, 'f');
}

fn another_function(x: i32) {
    println!("Another function. X is {x}");
}

fn measurement(value: i32, unit_label: char) {
    println!("Hello, you've measured {value} of {unit_label}")
}