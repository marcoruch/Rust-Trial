/*
* In this Application we explore Variables
*/
fn main() {


    let my_first_variable;

    my_first_variable = 3;

    // To print either pass a String-Type or use a format
    println!("{}", my_first_variable * my_first_variable);


    // let my_secondVariable => This would throw a awarning. Variables are declared with underscores instead of casing
    let my_second_variable = my_first_variable * my_first_variable;


    // implicit types
    let my_implicit_variable: i32 = 10;

    println!("First {0} - Second {1} - Implicit {2}", my_first_variable, my_second_variable, my_implicit_variable)
}