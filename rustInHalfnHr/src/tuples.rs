fn main(){
    //Rust has tuples, which you can think of as "fixed-length collections of values of different types"
    let pair = ('a', 17); // here this tuple has a character and a number together, ie a collection of different types
    // let pair: (char, i32) = ('a', 17); // anotate

    println!("{}", type_of(&pair));
    fn type_of<T>(_ : &T) -> String { // here & will not consume the string
        format!("{}", std::any::type_name::<T>())
    }
    println!("{}", pair.0);
    println!("{}", pair.1);
}
