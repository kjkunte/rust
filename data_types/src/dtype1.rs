fn main() {
    // let v1 = 251_u8 + 8; // this is an overflow as unsigned u8 would hold a value of 255 bits only
    let v1 = 251_u16 + 8;
    println!("{}", v1);
}