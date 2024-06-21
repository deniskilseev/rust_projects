fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // cannot multiply
    // let fp_times_i = 43.2 * 3;
    // but below is possible
    let fp_times_i = 43.2 * 3 as f64;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // can use pattern matching to destructure a tuple
    // like python's tup = (1, 2)
    // x, y = tup
    let (x, y, z) = tup;

    // accessing individual elements of tuple.
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of y is: {y}");

    let aa = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // let arr: [type; num_el]
    let aa: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // will create [3,3,3,3,3]

    let first = a[0];
    let second = a[1];
}