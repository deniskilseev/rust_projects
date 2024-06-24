fn main() {

    let other_s = "hello"; // known size @ compile time. stored on stack
    let s = String::from("hello"); // stored on heap
    
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`


    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
}
