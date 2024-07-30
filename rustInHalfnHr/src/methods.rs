struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { //Method rectangle 
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main(){
    let mut rect = Rectangle { width:10, height:20}; // instance/Obj of the struct rectangle
    println!("Area: {}", rect.area());
    rect.set_width(15);
    println!("New Width: {}", rect.width);
    println!("New Area: {}", rect.area());
}





