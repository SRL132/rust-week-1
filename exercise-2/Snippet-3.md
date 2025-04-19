```rust
fn main() {
    let x = 1;

    {
        let mut x = 2;

        x = x ^ 2;

        {
            x = 3;
            let x = 12;
        }
        println!("x is: {}", x);
    }
}
```

## Without executing the code, what is the printed value of x?
Printed value is x ^ 2 hence 4

## Test it and explain why x has this value.
It has this value because print is done in the scope where the last variable change is x = x ^ 2