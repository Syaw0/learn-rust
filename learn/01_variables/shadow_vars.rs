// * What is Shadow Variables ?
// * it's so simple , it's just redeclare the variable!

// ! Remember we can't redeclare `const` variables!!!

const KEY: i32 = 1451;
// ! We got error! `const` only can declare once at time
// const KEY: i32 = 212;

fn main() {
    // x is 1
    let x = 1;

    // * As you can see we can redeclare the `x` again with the new value
    let x = x + 2;

    {
        // it's block of code , no worries we reach that in future...

        let x = x + 10;
        println!("in the block x is : {x}"); // --> 13
    }

    println!("x should be 1 or 3 or what | result : {x}"); // --> 3

    println!("this is key constant : {KEY}");

    //

    // * Why we`redeclare` our variables instead of using `mut`
    // * when we redeclare variables it's made chance to change
    // * the variable type!!

    let space = "   "; // Its String
    let space = space.len(); // But now it's Number
    println!("the len of space is : {}", space);

    // what happen if we use mut?? let's see

    let mut space_2 = "   ";
    space_2 = space_2.len();
    // ! As you can see because `space_2` is first declared as `String`
    // ! Reassigned as `Number` give us a error!
}
