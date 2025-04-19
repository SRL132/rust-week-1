```solidity
// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

contract TestMapping {

    mapping(string => uint256) values;

    function add(string calldata input, uint256 value) external {
        values[input] = value;
    }

    function read(string calldata input) external view returns (uint256) {
        return values[input];
    }
}
```

```rust
use std::collections::HashMap;

fn main() {
    let mut values: HashMap<String, u64> = HashMap::new();

    values.insert(String::from("test"), 12345);
    println!("\"test\" associated value is {}", values.get(&String::from("test")).unwrap());
}
```

## What is the main difference between the two languages about non-initialized data?
Solidity mapping has a pre-defined behaviour where accessing a non-existing value it does not throw an error and just returns 0 without reverting

Rust behaviour can be customized, by default it reverts if it tries to access a non-existing value but if using unwap_or it can also return just a None value