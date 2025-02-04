fn main() {
   // let num = 7;

   // if else
   // if num != 7 {
   //     println!("num is not 7")
   // } else {
   //     println!("num is 7")
   // }

   // else if
   // if num % 4 == 0 {
   //    println!("Number is divisible by 4")
   // } else if num % 3 == 0 {
   //     println!("Number is divisible by 3")
   // } else if num % 2 == 0 {
   //     println!("Number is divisible by 2")
   // } else {
   //     println!("Number is not divisible by 4 3 2")
   // }

   // useing if in a let statement

   let condition = true;
   // if is an expression,
   // so we can use it on the right side,
   // of an let
   let number = if  condition{ 5 } else {6};

   println!("The value of number is: {number}")
}

// type of each arms in if statement,
// should be same,
// else rust will throw an error
