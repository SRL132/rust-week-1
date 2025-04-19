```rust
fn main() {
    let a = vec![1,2,3,4];
    a.push(27);
}
```

## Without testing it, what is wrong with this code snippet?
We are modifying an immutable variable

## How can it be fixed?
By changing a to mut

