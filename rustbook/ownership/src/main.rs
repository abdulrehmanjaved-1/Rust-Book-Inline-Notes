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
}
