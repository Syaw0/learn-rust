// ! You can't use `let` in global scope
// ! but we can use `const` instead
// let some_key = 123;

fn main() {
    let immutable_variable = 6;
    println!("{}", immutable_variable);

    // ! Got error !!
    // ! We can't assign a value twice to immutable variable
    // immutable_variable = 10;
    // println!("{}", immutable_variable)

    // * To fix this we can use `mut` keyword

    let mut mutable_variable = 11;
    println!("{}", mutable_variable); // --> 6

    mutable_variable = 10;
    println!("{}", mutable_variable); // --> 10
}
