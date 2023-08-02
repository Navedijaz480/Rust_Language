use std::io;
fn main(){
    println!("Enter any number ");
    let mut number = String::new();
   io::stdin().read_line(&mut number).unwrap();
   let numbers: i32 = number.trim().parse().unwrap();
    println!("the number is {}", number);
    table_of_number(numbers);
}
fn table_of_number(numbers :i32){
 for i in 1.. 11 {
    println!("{} * {} = {}", numbers , i ,i*numbers);
 }
}
