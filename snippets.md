

```rust
// Read i32 with cmd
// NOTE: invalid int input will be converted to 0
let mut input_int = String::new();
std::io::stdin().read_line(&mut input_int).ok().expect("Failed to read input int");
let input_int: i32 = input_int.trim().parse().unwrap_or(0);

// Read str with cmd
let mut input_str = String::new();
std::io::stdin().read_line(&mut input_str).expect("Failed to read line");
let input_str = input_strs.trim();
```