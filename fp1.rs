fn main(){
    let x: i32 = 5;
    assert_eq!("i32".to_string(), type_of(&x));
    println!("{}", type_of(&x));
    println!("Success!");
}

fn type_of<T>(_ : &T) -> String {
    println!("{}", std::any::type_name::<T>());
    format!("{}", std::any::type_name::<T>())
}
