fn main(){
    println!("Hello!!");
    // rust do not care about where we call our function before initializing it or after
    another_function(50, '$')
}
// unit label is a type 'char', this func accept
// two parameters
fn another_function(y: i32, unit_label: char){
    call_seven();
    let p = plus_one(20);
    println!("the value of p: {p}");
    println!("the value of y is: {y}{unit_label}")
}


// functions retirning value
// when we use ->, the function will return a value, and we have to declare data type
// body of the function is lonely semicolon,
//which means its an expression
fn seven() -> i32 {
    7
}

fn call_seven() {
   let x = seven();
   println!("The value of x is: {x}")
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// if something return a value then it is an expression
// if something do not return an value then it is an statement
// if there is no semicolon at the end then it will return an value.
// if it has semicolon then this will not return a value
