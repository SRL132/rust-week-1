

fn function_1(var: String) {

    println!("In function_1, variable is: {}", var);
}


fn main() {
    let variable = String::from("Welcome to RustSkills");

    function_1(variable);

    println!("In main, variable is: {}", variable);
}

//PROBLEM:
//owner of variable is scope main and we are tranferring ownership to function_1
//so when we print variable in main, it is not longer in 

//SOLUTION 1:
// apply borrowing to variable

fn function_1(var: &String) {
    println!("In function_1, variable is: {}", var);
}

//SOLUTION 2:
// return the variable from function_1 and assign it to variable in main

fn function_1(var: String) -> String {
    println!("In function_1, variable is: {}", var);
    var
}

//Now, replace the String variable with a scalar variable (u32, i32, u64, i64, …) and retest the same code snippet, why does it work?
//Because scalar variables have predictable size and are stored on the stack, not the heap, so rust doesn't need to apply ownership rules due to lack of predictability

