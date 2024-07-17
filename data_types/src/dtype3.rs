fn main(){
    let x: f64 = 1_000.000_1;
    let y: f32 = 0.12;
    let z: f64 = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success");
}

fn type_of<T>(_:&T) -> String { // this tells that the return type of the function type_of<T>  would be a String. <T> is a placeholder can accept reference of anytype
    println!("{}", format!("{}", std::any::type_name::<T>())); 
    format!("{}", std::any::type_name::<T>())
}