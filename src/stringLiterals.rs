fn main(){
    let string = String::from("Pakistan   ");
    let string_literals = &string[4..];
    let string_trim = &string.trim();

    dbg!(&string);
    dbg!(string.len());
    dbg!(string_literals);
    dbg!(string_trim.len());

    println!("string is  {}", string);




    let string1= String::from("USA...");
    let stringBorrow= string1;

    println!("string1 is {} ",stringBorrow);
    println!("string1  is after borrowed  {}",string1);
}