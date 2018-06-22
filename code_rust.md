```rust
// Contains Rust Examples Code Snippet.
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Array & Tuple:
fn main() {
    let array = [1, 2, 3, 4, 5];
    let tup: (i32, f32, char, bool) = (5, 1.4, 'A', false);
    println!("{:?}", array);
    println!("{:?}", tup);
}
// [16:19:23 abhasker@wsl -> hello$ cargo run
// [1, 2, 3, 4, 5]
// (5, 1.4, 'A', false)
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Function Parameter:
fn main() {
    another_function(5,6);
}
fn another_function(x: i32, y: i32) {
    println!("The value of (x,y) is: ({},{})", x, y);
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Using if in a let statement:
fn main() {
    let condition = true;
    let number = if condition {
                    5
                } else {
                    6
                };
    println!("The value of number is: {}", number);
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// while Loop:
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// for Loop:
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Difference between std::string::String & str 
fn main() {
    let mut s = String::from("hello");  // std::string::String
    let mut m = "hello";                // str : String literal

    s.push_str(", world!"); // OK
    m.push_str(", world!"); // ERROR : error[E0599]: no method named `push_str` found for type `&str` in the current scope
    // Because m type is str & push_str does not works on str.

    println!("s: {}", s);   // OK
    println!("m: {}", m);   // ERROR : error[E0599]
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Rust Slice Data Type Example with Removed Error:
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("word: {}", word);
    // s.clear(); // ERROR: error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //  ^ mutable borrow occurs here
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];    // Slice of String s.
        }
    }
    &s[..]
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Rust struct Example:
extern crate time;
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: time::Tm,
        active: bool,
        phone: String,
    }
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: test(),
        phone: String::from("phone_number"),
    };

    user1.email = String::from("anotheremail@example.com");
    println!("email: {:?}", user1.email);

    // With user1.sign_in_count it returns the struct of type time::Tm written above in struct definition.
    // println!("sign_in_count: {:?}", user1.sign_in_count);
    // sign_in_count: Tm { tm_sec: 11, tm_min: 59, tm_hour: 21, tm_mday: 18, tm_mon: 5, tm_year: 118, tm_wday: 1, tm_yday: 168, tm_isdst: 0, tm_utcoff: 19800, tm_nsec: 320147300 }
    println!("sign_in_count: {:?}", user1.sign_in_count.tm_nsec);
    println!("active: {:?}", user1.active);
}

fn test()-> time::Tm{
    return time::now();
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Rust struct and method with Debug derived trait implementation Example:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//  Implement the impl (Implementation) block wth the struct name.
impl Rectangle {            
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()    //  Method syntax to call area.
    );
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


























```



