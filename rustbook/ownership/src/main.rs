fn main() {
    //stack copy value
    // let x: &str="hello"; //string literal non expandable
    // let y: &str=x;
    // println!("the value of y and x is {} {}",x,y);

    //heap does not copy
    // let x: String=String::from("Hello"); //heap variable which contains
    //ptr which is pointer and live in stack.
    //length 
    //capacity
    //in heap case when we make a=b then content of String does not get copied as it is expensive 
    //operation to allocate again for a, it just make a pointer to that.
    // let y: String=x;
    // let z: String=x;
    //there can be only one woner of a variable when it goes out of scope then that not 
    //valid. you have to do call clone() method for deep copy and default one is move not 
    // shallow. Also when heap String passed to the functions it takes ownership.

    let mut s: String = String::from("Abdulrehman Javed");
    let len: usize = calculate_len(&s); //Borrowed by & //at this moment we have passed the ownership of s to calculate_len
    //which mean calculate_len is responsible to drop() its scop to clear memory now. 
    println!("the values of s and len are {} {}",s , len);
    change_me(&mut s);
    println!("the values of s appended is {}",s);
    


}
//lets make a calculate_len functions which also returns ownership after doing its job.
// fn calculate_len(s: String) -> (String, usize) {
//     let result: usize=s.len();
//     (s, result) //returning tuple code
// }

//what is referencing and Borrowing??
//instead of moving  the s value, we can pass just refernce so we dont move the scope. for example: 
fn calculate_len(s: &String) -> usize {
    return s.len();
}

//how other function can mutate the value not just read as it was in only refernce case, it cannot modify using *(deref)
fn change_me(s: &mut String) -> () {
    s.push_str(" change append");
}

//also note about mutable refence that if you have a mutable reference to a value, you can have no other ref to tht value.
// fn test_fn() -> (){
//     let mut s1: String= String::from("hello");

//     let s2= &mut s1;
//     let s3= &mut s1;

//     s2.push_str("string");
// }



