
```rust

Rust Notes:

$ sudo /home/abhasker/.cargo/bin/cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index

#	println is macro not a function.
println!("Please input your guess.");

#	In Rust, variables are immutable by default.

#	Declare new string type mutable data type with let.
let mut guess = String::new();		//	An associated function

#	The :: syntax in the ::new line indicates that new is an associated function of the String type.

#	An associated function is implemented on a type (data type??), in this case String, rather than on a particular instance of a String.
#	Some languages call this a static method (Fucking java!!).

let foo = 5;		// immutable variable comment
let mut bar = 5;	// mutable variable comment

#	Include Standard Library:
#	Otherwise use to call function io::stdin()
use std::io;
OR
std::io::stdin

#	use in Rust: works just like Perl, we can use it anywhere.
#	Ordering of this line will not matter just like print Dumper($var); use Data::Dumper;

#	The & indicates that this argument is a reference, which gives you a way to let multiple parts
#	of your code access one piece of data without needing to copy that data into memory multiple times.

# Read from Terminal contains newline char.
io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
OR
io::stdin().read_line(&mut guess).expect("Failed to read line");

#	read_line puts what the user types into the string we’re passing it, but it also returns a value—in this case,
#	an io::Result. Rust has a number of types named. Result in its standard library: a generic Result as well as specic
#	versions for submodules, such as io::Result.

#	The Result types are enumerations, often referred to as enums.
#	An enumeration is a type that can have a fixed set of values.

#	TO Remove it we can use trim_right function, will remove all extra chars like space, tab, newline char.
#	Not like chomp subroutine.
let guess = guess.trim_right();

#	Printing values with println! Placeholders:
println!("You guessed: {}", guess);

$ cargo run
Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
5
You guessed: 5

#	crate is a package of Rust code.
#	The rand crate is a library crate, which contains code intended to be used in other programs.

Filename: Cargo.toml
[dependencies]
rand = "0.3.14"
OR
rand = "0.*"	// Install latest rand crate automatically.

$ cargo build
Updating registry `https://github.com/rust-lang/crates.io-index`
Downloading rand v0.3.14
Downloading libc v0.2.14
Compiling libc v0.2.14
Compiling rand v0.3.14
Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs

#	The [dependencies] section is where you tell Cargo which external crates your project depends on and which versions of those crates you require.

#	Semantic Versioning (sometimes called SemVer):
semantic version specifier:
0.3.14

$ cargo build
Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs

$ cargo update
Updating registry `https://github.com/rust-lang/crates.io-index`
Updating rand v0.3.14 -> v0.3.15

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 3
Please input your guess.
1
You guessed: 1
Too small!

extern crate rand;
#	We add a line that lets Rust know we’ll be using the rand crate as an external dependency.
#	This also does the equivalent of calling use rand, so now we can call anything
#	in the rand crate by placing rand:: before it.

use rand::Rng;
#	We add another use line: use rand::Rng.
#	The Rng trait defines methods that random number generators implement,
#	and this trait must be in scope for us to use those methods.


#	The gen_range method takes two numbers as arguments and generates a random number between them.
#	It’s inclusive on the lower bound but exclusive on the upper bound
let secret_number = rand::thread_rng().gen_range(1, 5);


match guess.cmp(&secret_number) {		// &secret_number refers to secret_number reference.
	Ordering::Less => println!("Too small!"),
	Ordering::Greater => println!("Too big!"),
	Ordering::Equal => println!("You win!"),
}

#	Ordering is another enum, but the
#	variants for Ordering are Less, Greater, and Equal.

#	The cmp method compares two values and can be called on anything that can be compared.
#	It takes a reference to whatever you want to compare with: here it’s comparing the guess to the secret_number.
#	Then it returns a variant of the Ordering enum we brought into scope with the use statement.
#	We use a match expression to decide what to do next based on which variant of Ordering was returned
#	from the call to cmp with the values in guess and secret_number.

#	Interger Constraint on guess input, Otherwise will get an error : "Please type a number!"
let guess: u32 = guess.trim().parse().expect("Please type a number!");

let mut secret_number = String::new();					//	with reference type `&std::string::String`
let secret_number = "Hello";							//	with reference type `&&str`

####	Rust Debugging:
#	As far as debugging goes, a little trick:
let x = ......; // don't know the type of x
let y: () = x;  // will throw a type error with the name of x's type

#	() => Empty Tuple

#	Example :
	let secret_number = rand::thread_rng().gen_range(1, 5);		// Didn't know this was integer.
	let type_of_variable: () = secret_number;					// lets find out.
expected (), found integral variable							// Result: integral variable
    let mut guess = String::new();								// Didn't know this was complex object std::string::String.
    let y: () = guess;											// lets find out.
expected (), found struct `std::string::String`					// Result: integral variable, as expected.

#	Rust Integer defaults to an i32 a 32-bit number;
#	Rust cannot compare a string and a number type.
#	Rust allows us to shadow the previous value of variable with a new type.
let secret_number = "Hello";		//	String type variable.
let secret_number = 2;				//	Shadow the variable secret_number from string to integer by using let again.
#	let guess: u32. The colon ( : ) after guess tells Rust we’ll annotate means declare the variable’s type.

#	The loop keyword creates an infinite loop like while (1), Use exit condition to break from the loop.
loop {
	println!("Please input your guess.");
	match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => println!("You win!"),
	}
}

#	Exiting from Infinite loop with break keyword:
loop {
	println!("Please input your guess.");
	match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => {
			println!("You win!");
			break;
		}
	}
}

#	Replaced expect to match here to continue with the non-number input:
#	The underscore, _ , is a catchall value;
let guess: u32 = guess.trim().parse().expect("Please type a number!");
let guess: u32 = match guess.trim().parse() {
	Ok(num) => num,
	Err(_) => continue,
};

#	continue keyword must be inside a loop.

#####	Chapter: 3 Common Programming Concepts		#####

##	Differences Between Variables and Constants

#	Can't use mut keyword with constants const keyword.
#	You declare constants using the const keyword instead of the let keyword.
#	Constants can be declared in any scope, including the global scope.
#	Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

const PI = 22 / 7;		// OK : Constant Expression.
const mut PI = pi();	// NOT OK: ERROR (Can't use mut keyword with const.)
const PI = pi();		// NOT OK: ERROR (Only Constant Expression allowed.)
let const PI = pi();	// NOT OK: ERROR (Can't use let keyword with const.)

#	Rust’s constant naming convention is to use all uppercase with underscores between words ie. (MAX_POINTS).
#	Declaration of constants:
const MAX_POINTS: u32 = 100_000_101;	// "_" is used here to increase readbility of numbers, Compiler ignores it completely.

##	Shadowing
#	Re-declare a variable using let again to shadow the previuos type & value of data type.

#	Shadowing is different than marking a variable as mut. because??
#	We’ll get a compile-time error if we accidentally try to reassign to any variable without using the let keyword.

#	The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again.
#	We can change the type of the value but reuse the same name. saves us from the heckle of using temp name.

#	Shadowing thus spares us from having to come up with different names.
let spaces = " ";
let spaces = spaces.len();			// Using let again allows us to change the variable type from string to usize here.

#	We aren't allowed to change a mut variable type.
let mut spaces = " ";				// variable mut spaces type is string here.
let spaces = spaces.len();			// Here we are trying to change this data type usize, It doesn't matter if variable is mutable or not in first case.
=>	will give error: expected &str, found usize

##	Data Types:

#	Data type subsets:
1. Scalar
2. Compound

#	Rust is a statically typed language.
#	Which means that it must know the types of all variables at compile time.

#	Scalar Types:
#		A scalar type represents a single value.
#		Rust has four primary scalar types:
		1.	integers
		2.	foating-point numbers
		3.	booleans
		4.	characters


#	Integer Types:
u = unsigned integer types
i = signed integer types

Length		Signed		Unsigned
8-bit 		i8 			u8
16-bit 		i16 		u16
32-bit 		i32 		u32
64-bit 		i64 		u64
arch 		isize 		usize

#	Integer literals Form
Number literals 		Example
Decimal 				98_222
Hex 					0xff
Octal 					0o77
Binary 					0b1111_0000
Byte(u8 only)			b'A'

#	Signed variant size:  -(2ⁿ - 1) to +(2ⁿ⁻¹ - 1) inclusive.

#	Floating-Point Types: IEEE-754 standard
f32 Single Precison Float
f64 Double Precison Float
#	Default : f64

#	Can't use integer value in declartion expression.
let var_float: f32 = 2.1 / 4.546;	// OK
let var_float: f32 = 2   / 4.546;	// ERROR : cannot divide `{integer}` by `{float}`


#	Boolean Type:
#	The bool represents a value, which could only be either true or false.
#	If you cast a bool into an integer, true will be 1 and false will be 0.
let t = true;
let f: bool = false; // with explicit type annotation

#	Character Type:
#	char type is specified with '' single quotes, While String uses "" double quotes.
let tick  = '✔';
let heart = '❤';

#	char type represents a Unicode Scalar Value.
#	char is a Unicode scalar value, which is similar to, but not the same as, a 'Unicode code point'.
#	Unicode Scalar Values range: U+0000 to U+D7FF && U+E000 to U+10FFFF inclusive.

#	Compound Types:
#	Compound types can group multiple values into one type.


#	The Tuple Type:
#	A tuple is a general way of grouping together some number of other values with a variety of types into one compound type.
let tup: (i32, f64, u8) = (500, 6.4, 1);	// tup single compound variable.
#	We create a tuple by writing a comma-separated list of values inside parentheses.
#	Each position in the tuple has a type.
#	Types of the different values in the tuple don’t have to be the same.


#	To get the individual values out of a tuple.
#	We can use pattern matching to destructure a tuple value.
let tup = (500, 6.4, 1);
let (var1, var2, var3) = tup;	// Rust destructuring of tuple type variable.
OR
let tup_var1 = tup.0;	// Gets 500 as value.
let tup_var2 = tup.1;	// Gets 6.4 as value, Same as var3.
let tup_var3 = tup.2;	// Gets 1 as value.
println!("The value of var3 is: {}", var3); // "The value of var3 is: 6.4".
#	This is called destructuring, because it breaks the single tuple into three parts.


#	The Array Type:
let a = [1, 2, 3, 4, 5];
#	Every element of an array must have the same type unlike Tuple variable.
#	Declaration syntax also differs from tuple.
#	Arrays in Rust are different from arrays in some other languages like perl.
#	Arrays in Rust have a fixed length: once declared, they cannot grow or shrink in size.

#	Arrays are useful when you want your data allocated on the stack rather than the heap (perfomance gain.)

#	Array elements access:
let first  = a[0];
let second = a[1];

let index   = 10;
let element = a[index];	// Invalid Memory Access not allowed. Runtime error
Runtime: ERROR: thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is 10'

#	Functions:
#	Rust code uses snake case (variable_name) as the conventional style for function and variable names.

#	Function Parameters:
another_function(5, 6);
fn another_function(x: i32, y: i32) {	// x type signed interger 32.

#	In function signatures, you must declare the type of each parameter.

#	Function Bodies:
#	Function bodies are made up of a series of statements optionally ending in an expression.

#	Statements and Expressions:

#	Statements : are instructions that perform some action and do not return a value.
#	let y = 6; is a statement.
#	Statements do not return values. Therefore, you can’t assign a let statement to another variable.
let x = (let y = 6); // NOT OK: ERROR: error: expected expression, found statement (`let`)
// The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.

#	Expressions : evaluate to a resulting value.
#	Expressions do not include ending semicolons.
5 * 6	// wihtout ; at the end: Expression.
5 * 6;	// with ; at the end: Statements.
#	 If you add a semicolon to the end of an expression, you turn it into a statement.
#	Expressions can be part of statements.
#	Calling a function is an expression.
#	Calling a macro is an expression.
#	The block that we use to create new scopes, {} , is an expression.

fn main() {
	let x = 5;
	let y = {
		let x = 3;	// y = { } is a expression, evaluates to 4.
		x + 1		// Doesn't include ; at the end, so it's an expression not a statement.
	}; //  If you add a semicolon to the end of an expression, you turn it into a statement.
	println!("The value of y is: {}", y);
}

#	Functions with Return Values:
#	We don’t name return values, but we do declare their type after an arrow ( -> ).
let x = five();
fn five() -> i32 {	// fn fn_name() -> return_data_type { }
	5	// Without ; returns value because of the expression.
}

#	In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
#	You can return early from a function by using the return keyword and specifying a value,
#	But most functions return the last expression implicitly.

#	Functions also have a type! They look like this:
fn foo(x: i32) -> i32 { x }
let x: fn(i32) -> i32 = foo;
#	In this case, x is a ‘function pointer’ to a function that takes an i32 and returns an i32.

#	When statements don’t evaluate to a value, which is expressed by (), the empty tuple.

#	Control Flow

#	if Expressions:
#	Condition in this code must be a bool. If the condition isn’t a bool, We’ll get an error.
let number = 3;
if number {			// NOT OK: ERROR: ^^^^^^ expected bool, found integral variable.
if number != 0 {	// OK

#	if else Block:
if {
}
else if{
}
else{
}

#	Using if in a let Statement:
#	Because if is an expression, we can use it on the right side of a let statement.
let number = if condition {
				5
			} else {
				6
			};

#	match can also use simple logical expressions in its arms.
let x = 4;
match x {
    3|5|6  => { println("First arm!"); }
    10..16 => { println("Second arm!"); }
    _      => { println("Default arm!"); }
}

#	This means the values that have the potential to be results from each arm of the if must be the same type.

#	Repetition with Loops:
#	Rust has three kinds of loops: loop , while , and for.

#	loop:
#	Repeating Code with loop
#	The loop keyword tells Rust to execute a block of code over and over again.
#	forever or until you explicitly tell it to stop.

#	while:
#	This while loop approch here in this case is error prone.
#	we could cause the program to panic if the index length is incorrect.

#	It’s (while?) also slow, because the compiler adds runtime code to perform.
#	The conditional check on every element on every iteration through the loop.

// while Loop:
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}


#	for:
#	Use a for loop and execute some code for each item in a collection.

// for Loop:
fn main() {
    let a = [10, 20, 30, 40, 50];	// Rust Array type.

    for element in a.iter() {	// Returns a collection.
        println!("the value is: {}", element);
    }
}

#	Safe Approch:
#	Run some code certain times approch:
#	would be to use a Range, which is a type provided by the standard library
#	that generates all numbers in sequence starting from one number and ending before another number.

#####	Chapter: 4 Understanding ownership	#####

#	Stack:
#	The stack stores values in the order it gets them and removes the values in the opposite order.
#	This is referred to as last in, first out.
#	Adding data is called pushing onto the stack, and removing data is called popping off the stack.
#	The stack is fast. Heap is Slow. All data on the stack must take up a known, fixed size.

#	Heap:
#	Data with a size unknown at compile time or a size that might change can be stored on the heap instead.


#	Ownership Rules

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

#	If we want to take user input and store it: use String.
#	Rust has a second string type: String

let s = String::from("hello");

#	This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.

#	The double colon ( :: ) is an operator that allows us to namespace this particular from
#	function under the String type rather than using some sort of name like string_from.


#	This kind of string can be mutated:
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`


// Difference between std::string::String & str
fn main() {
    let mut s = String::from("hello");  // std::string::String
    let mut m = "hello";                // str : String literal

    s.push_str(", world!"); // OK
    m.push_str(", world!"); // ERROR : error[E0599]: no method named `push_str` found for type `&str` in the current scope
    // Because m type is str & push_str does not works on str.

    println!("s: {}", s);   // OK
    println!("m: {}", m);   // ERROR
}

#	Why can String be mutated but Literals cannot?
--	How these two types deal with memory.
--	String on Heap, Literals on Stack.

#	drop: When a variable goes out of scope, Rust calls a special function for us. 
#	This function is called drop.

let s1 = String::from("hello");
let s2 = s1;

#	Double free error:
In above code s1 is moved to s2 due to rust out of scope logic,
Otherwise if s2 = s1 both point to same memory location, rust will try to free the same memory known as Double free error.

#	So, Instead of trying to copy the allocated memory (s2 = s1), Rust considers s1 to no longer be valid.
#	Therefore, Rust doesn’t need to free anything when s1 goes out of scope.

----------------------------------------------------------------------------------------------------
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
----------------------------------------------------------------------------------------------------

#	Rust will never automatically create "deep" copies of your data.

#	Ways Variables and Data Interact: Clone:

#	If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);

#	Stack-Only Data: Copy:
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);	//	OK: Because x lives on stack (integers that have a known size at compile time are stored entirely on the stack).

#	traits: more on this later.
#	Copy: If a type has the Copy trait, an older variable is still usable after assignment.
#	Drop: Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait.
#	If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type.

#	General rule: Any group of simple scalar values can be Copy, and nothing that requires allocation or is some form of resource is Copy. 

#	Here are some of the types that are Copy:
	#	All the integer types, such as u32.
	#	The Boolean type, bool, with values true and false.
	#	All the floating point types, such as f64.
	#	The character type, char.
	#	Tuples, but only if they contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.
#

#	Ownership and Functions:
#	The semantics for passing a value to a function are similar to those for assigning a value to a variable. 
#	Passing a variable to a function will move or copy, just as assignment does.

#	Meaning by example:
#	When s is passed on to the function, It goes out of scope, So Ownership works on assigning a value to a variable as well as passing a value to function.

let s = String::from("hello");  // s comes into scope.
takes_ownership(s);             // s's value moves into the function...
								// ... and so is no longer valid here.
println!("{}", s)				// NOT OK


#	Return Values and Scope:
#	Returning values can also transfer ownership.

let s1 = gives_ownership(); // gives_ownership moves its return, value into s1.

fn gives_ownership() -> String {	// gives_ownership will move its, return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string	// some_string is returned and moves out to the calling function.
}

#	It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again,
#	In addition to any data resulting from the body of the function that we might want to return as well.


#	Return Multiple Values: from function.
#	It’s possible to return multiple values using a tuple:

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

#	Too much work: Luckily for us, Rust has a feature for this concept, called references.


#	References and Borrowing

#	references:
#	These ampersands are references, and they allow you to refer to some value without taking ownership of it.
#	Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

let s1 = String::from("hello");
let s2 = calculate_length(&s1);	// &s1 reference to String Object s1.
println!("The length of '{}' is {}.", s1, len);	// OK

fn calculate_length(s: &String) -> usize {	// the signature of the function uses & to indicate that the type of the parameter s is a reference.
    s.len()
}

#	The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
#	Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.


#	When functions have references as parameters instead of the actual values.
#	We won’t need to return the values in order to give back ownership, because we never had ownership.


#	references as function parameters : borrowing

#	What happens if we try to modify something we’re borrowing?
	#	It doesn't work. // ERROR : Attempting to modify a borrowed value
#

#	Just as variables are immutable by default, so are references.
#	We’re not allowed to modify something we have a reference to.

#	Mutable References:
change(&s); => change(&mut s); 
&& 
fn change(some_string: &String) { => fn change(some_string: &mut String) { to modify the reference value.
#	First, we had to change s to be mut. Then we had to create a mutable reference with &mut s and accept a mutable reference with some_string: &mut String.

#	But mutable references have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope.

let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;	// NOT OK: ERROR: error[E0499]: cannot borrow `s` as mutable more than once at a time.

#	With the above error : Rust can prevent data races at compile time.

#	 A data race is similar to a race condition and happens when these three behaviors occur:
	#	Two or more pointers access the same data at the same time.
	#	At least one of the pointers is being used to write to the data.
	#	There’s no mechanism being used to synchronize access to the data.
#

#	we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.
let r2 = &mut s;

#	Dangling References:
#	A pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory.
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

#	The Rules of References
	#	At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
	#	References must always be valid. No dangling.
#


#	The Slice Type
#	slice data type does not have ownership.
#	Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. (Like perl array slice.)

#	String slices Example:

let s = String::from("hello");
let slice = &s[0..2];	//	Equal
let slice = &s[..2];	//	Equal

let s = String::from("hello");
let len = s.len();
let slice = &s[3..len];	//	Equal
let slice = &s[3..];	//	Equal

#	You can also drop both values to take a slice of the entire string. So these are equal:
let s = String::from("hello");
let len = s.len();
let slice = &s[0..len];	//	Equal
let slice = &s[..];		//	Equal


#	Other slices example:
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];	// This slice has the type &[i32], Used in mostly collections.

####	Chapter: 5 Using Structs to Structure Related Data	####
























































```
