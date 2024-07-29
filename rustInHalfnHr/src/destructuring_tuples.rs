fn main() {
    // let (left, right) = slice.split_at("middle")
    let slice = &[1,2,3,4,5];
    let middle = 2;
    let(left, right) =  slice.split_at(middle);
    //fn split_at(&self, mid: usize) -> (&[T], &[T])
    //&self is a reference to the instance of the slice method is being called on
//  The method returns a tuple containing two slices

    println!("right slice: {:?}", right);
    println!("left slice: {:?}", left);
}






