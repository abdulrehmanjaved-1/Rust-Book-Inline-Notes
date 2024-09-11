// use std::any::type_name_of_val;

use std::string;

fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("the value of const is: {THREE_HOURS_IN_SECONDS}");
    //mutate concept
    // let mut spaces: &str = "   ";
    // println!("here I am using spaces: {spaces} hello");
    // spaces=" ";
    // println!("here I am using spaces: {spaces} hello");
    
    //shadowing concept
    let spaces: &str = "   ";               // `spaces` is a &str
    let spaces: usize = spaces.len();       // `spaces` is now shadowed as a usize
    let type_of_spaces: &str = std::any::type_name_of_val(&spaces);  // Determine the type of `spaces`
    println!("The type of `spaces` is {}", type_of_spaces); 

    //scalor and compoud types in rust
    //lets see compound types
    //tuples:
    // tuples are compound of different data type values
    // tuples cannot grow or shrink in size
    let tuple_1: (&str, i32, f64) = ("abdulrehman", 1, 1.3); //auto infering the data types
    let unit_tuple = (); //unit tuple which is empty for now
    //how we can access indiviidual points of this tuple?
    //first way is to destructuring.
    let (x,y,z)=tuple_1;
    // println!("the first value is {}", x);
    // println!("the second value is {}", y);
    // println!("the third value is {}", z);

    //second way is directly accessing by .
    println!("the fisrt value is {}", tuple_1.0);
    println!("the fisrt value is {}", tuple_1.1);
    println!("the fisrt value is {}", tuple_1.2);

    //next compound type is array type
    //in array elelments should be of same type
    let a: [i32; 4] =[1,2,3,4];
    //cannot shrink or expand its size
    //use vector if size is not confirmed
    //there is a syntax for something
    let b: [i32; 5] = [10; 5];
    println!("values in b array are {}", b[0]);


    //lets see functions now
    let random_value: i32=sum();
    println!("the value returned by the sum function is {}", random_value)
    //funxtion is a combination of statements and expressioss, expressions are anything that
    //returns something. let a=b+c so b+c is expression but whole is statement.
}

fn sum()-> i32{
    20 //style in rust to return  but if semi coloned then it will be a statement
}