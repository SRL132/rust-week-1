```rust
fn my_operation(a: u64, b: u64) -> u64 {
    a += b;
    a
}


fn main() {
    let num1 = 1234;
    let num2 = 1122;
    println!("My result is {}!", my_operation(num1, num2));
}
```

## Without testing it, what is wrong with this code snippet?
We are changing an immutable variable (a)

## How can it be fixed?
By changing a to mut

