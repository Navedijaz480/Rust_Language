// fn main() {
//     println!("Hello, SiliconNexus");
// }


// fn main(){
//     println!("hello silicon")
// }



// fn main() {
//     let answer = 42;
//     println!("Hello {}", answer);
// }



// fn main() {
//     let answer = 42;
//     assert_eq!(answer,42);
// }

//not equal 
// fn main() {
//     let answer = 40;
//     assert_eq!(answer,42);
// }




// Looping and Ifing

// for1.rs
// fn main() {
//     for i in 0..10 {
//         println!("Hello {}", i);
//     }
// }

// for2.rs
// fn main() {
//     for i in 0..5 {
//         if i % 2 == 0 {
//             println!("even {}", i);
//         } else {
//             println!("odd {}", i);
//         }
//     }
// }

// for3.rs
// fn main() {
//     for i in 0..5 {
//         let even_odd = if i % 2 == 0 {"even"} else {"odd"};
//         println!("{} {}", even_odd, i);
//     }
// }


// Adding Things Up
// Computers are very good at arithmetic. Here is a first attempt at adding all the numbers from 0 to 4:

// add1.rs
// fn main() {
//     let sum = 0;
//     for i in 0..5 {
//         sum =sum+i;
//     }
//     println!("sum is {}", sum);
// }


// add2.rs
// fn main() {
//     let mut sum = 0;
//     for i in 0..5 {
//         sum += i;
//     }
//     println!("sum is {}", sum);
// }

// add3.rs
// fn main() {
//     let mut sum = 0.0;
//     for i in 0..5 {
//         sum += i as f64;
//     }
//     println!("sum is {}", sum);
// }
// let guess: u32 = "42".parse().expect("Not a number!");

// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

// println!("the value of x is  {}",x);
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;


//     println!("the value is  {}",{difference});


// }


// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
//     println!("heart_eyes_cat {}",heart_eyed_cat );
// }


// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }



//store value in array 

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     // let five_hundred = x.0;

//     // let six_point_four = x.1;

//     // let one = x.2;
//     println!("five {}",x.1)
// }


// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let second = a[1];
//     println!("the value of a is {}",a[4]);
// }





// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }


// fun1.rs

//square fuction user defined 

// fn sqr(x: f64) -> f64 {
//     return x * x;
// }

// fn main() {
//     let res = sqr(2.5);
//     println!("square is {}", res);
// }

// fn by_ref(x: &i32) -> i32{
//     *x + 1
// }

// fn main() {
//     let i = 10;
//     let res1 = by_ref(&i);
//     let res2 = by_ref(&41);
//     println!("{} {}", res1,res2);
// }
// // 11 42

// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function. 12");
// }

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }



// fn main() {
//     let x = (let y = 6);
// }


// fn main() {
//     let y = 6;
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }


// array1.rs
// fn main() {
//     let arr = [10, 20, 30, 40];
//     let first = arr[0];
//     println!("first {}", first);

//     for i in 0..4 {
//         println!("[{}] = {}", i,arr[i]);
//     }
//     println!("length {}", arr.len());
// }


//function concept 

// fn main(){
// sum()
// }
// fn sum(){
//     println!("the sum is");
// }
// fn main(){
// //////////////////////////////sum(4,5);
// println!("the multiplication of two number is {}", sum(4,5));
// }
// fn sum(a : u64 , b : u64)->u64{
//    let c= a+b;
//    println!("the sum is {}", c);
//    let d = a*b;
//    return d;
//    //also can remove return keyword 
// }



//memory management 
// fn main (){
//     let num ;
//     println!("{}", num)
// }

 // declare variable with data type 

// fn main (){
//     let num:u8 =22 ;
//     println!("{}", num)
// }


// fn main (){
//     let num:u8 =256 ;
//     println!("{}", num)
// }




// touple in rust language for hetrogenius 


// fn main (){
//     let touple = (10, 1.5 ,'a');
//     println!("................. {}", touple.2);
// }

// fn main(){
//     let  mut count=0;
//     loop{
//         count+=1;
//         println!("hi {}", count);
//         if count ==20{
//             break;
//         }
//     }
// }



//input output library 
// use std::io;
// fn main()
// {
//     let mut buffer = String::new() ;
//     println!("enter message");
//     io::stdin().read_line(&mut  buffer);
//     println!("the buffer is {}", buffer)

// }



use std::io;
fn main(){
let mut buffer= String::new();
println!("enter the text please ");
io::stdin().read_line(&mut buffer);
println!("the buffer is {}", buffer);


}





































