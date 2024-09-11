pub mod memory {
    pub fn my_fn(){
        let fname = String::from("Abdul");
         println!("Name has been printed {}", fname);
    }
    println!("name is {}", fname);
    pub fn print_name(name: String){
        println!("Name is {}", name)
    }

}