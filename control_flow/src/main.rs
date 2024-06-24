fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number < 5 {
        println!("number less than five");
    } else if number < 2 {
        println!("nubmer less than two");
    } else {
        println!("something else");
    }
    //

    let condition = true;
    let x = 3;
    let number = if condition { 
        x + 3
    } else { 
        x + 4
    };
    
    println!("The value of number is: {number}");

    fn main() {
        let mut counter = 0;
    
        let result = loop {
            counter += 1;
    
            if counter == 10 {
                break counter * 2;
            }
        };
    
        println!("The result is {result}");
    }
}