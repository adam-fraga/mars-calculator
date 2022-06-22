use ::std::io;

/*
*   3 Ownership Rules:
*
*     1 - Each value in rust is own by a variable.
*     2 - When the owner goes out of the scope, the value will be deallocated;
*     3 - There can only be one owner at a time.
*/

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn main() {
    let mut input = String::new(); //String is the struct wich allo to create string

    println!("Enter your weight: (kg)");

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    dbg!(weight); //Debugging function show variable and type
    let mars_weight = calculate_weight_on_mars(weight);
    println!("On mars your weight is {}kg", mars_weight);

    let mut input2 = String::new();
    // let mut s = input;  (transfert ownership of input to s input does not exist since now)

    /*
    You can have as many mutable reference as you want in the same scope BUT you can only have ONE mutable
    reference in the same scope
    */

    let s1 = &input;
    // let mut s1 = &mut input; //Uncomment to see the errors
    let s2 = &input;

    /*
    If s1 is mutable the compiler will throw an error, you cannot have a mutable
    reference and a an other mutable or non mutable reference in the same time. This behavior prevent data
    races in compile time which is extremly powerfull
    */

    println!("{} {}", s1, s2); // Work if s1 & s2 are immutable ref

    /*
    Transfert ownership input to the function
    notice that input is no longer available outside "some_function()"
    end at the end of the function rust compiler deallocated input and we lost the value
    */
    some_function_take_ownership_of_value(input);

    // println!("{}", input); Uncomment to see the error

    /*
    Borrowing in contrary of transfering ownership provide a way to return value to continue
    working with it at the end of the function with references, you can specify that the ref is mutable.
    */
    some_function_borrowed_value(&input2);
    some_function_borrowed_mutable_value(&mut input2);
    io::stdin().read_line(&mut input2);
    // println!("Input: {}", input2);
    // mars_weight = mars_weight * 1000.0;
    // println!("Weight on mars: {}kg", mars_weight);
}

fn some_function_take_ownership_of_value(s: String) {} //Function take the ownership of the value
fn some_function_borrowed_value(s: &String) {} //Function borrowed value
fn some_function_borrowed_mutable_value(s: &mut String) {
    //You can modify borrowed value with "&mut"
    s.push_str("a");
}
