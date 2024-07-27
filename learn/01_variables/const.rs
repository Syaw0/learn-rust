// * you can use `const` to assign a constant in global scope
// * that mean these variables can used by almost any other scopes
// ! Don't forget , we should always annotate if we are using `const`
const GLOBAL_KEY: i32 = 112233;

// * It's better to use UPPERCASE naming for `const` variables
// * Note that all the constant with `const` calculated in `compile time`
// * not in the `run time` so we always must assign a expression to `const`
// * if it's could calculated in `compile`!!
const DELAY_TIME: i32 = 60 * 60 * 2;

fn main() {
    // * Note that we can use global constant variable inside almost
    // * any scopes!
    const PRIVATE_KEY: i32 = 44554;

    println!("this is global key : {}", GLOBAL_KEY);

    println!("this is main function private key : {}", PRIVATE_KEY)
}
