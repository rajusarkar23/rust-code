fn main(){
    // mut means this value is mutable,
    // in rust the values are immutable by default.
    let  x = 5;
    {
        let x = x * 2; // in rust this is called shadowing
        println!("The value of x in inner scope: {x}")
    }
    println!("The value of x is: {x}");
    println!("The value of x is: {x}")
}

// we can also use constant value in rust,
// to declare a constant we use const word just like javascript.
// in rust const variable naming cinvention is => uppercase with underscore
// difference between let mut variable_name and let variable_name is that
// when we use only let we are declaring a new varible