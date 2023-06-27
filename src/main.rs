// fn main() {
//     println!("Hello, world!");
// }



use rand :: random;
use std::io;
use std::cmp::Ordering;
fn main(){
    let number=random::<u8>();
    println!("{}", number);
    if number < 100 {
    println!(" you lost your number is less then hundred ");
    }
    else if number == 100{

    println!("you win");
    }
    else{

        println!("you lost your number is greater then hundred");
        }

        loop{
        println!("please guess a number ");
     let mut guess=String::new(); 
     io::stdin().read_line(&mut guess);


     println!("the guess number is {}", guess);

     let guess:u8= guess.trim().parse().expect("type an integer");
    //  if (guess==number) {
    //     println!("you are winner ");

    //  }
    //  else {
    //     println!("you are loser ");
    //  }
    match guess.cmp(&number){
    Ordering::Less=> println!("to less"),
    Ordering::Greater=> println!("to big"),
    Ordering::Equal =>
    {println!(" congratulation you win");
    break;
    },
}
        }
}