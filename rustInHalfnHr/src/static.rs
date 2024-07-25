// The static lifetime : There is a special lifetime, named static, which is valid for the entire program's lifetime
// String literals are static:

struct Person {
    name: &'static str,
    //her `'static` is a lifetime which means for the entire duration of the program
    // &'static str , adding an & will make this a reference 
    // here the name field is intended to hold the reference to a string literal
    // the string literal will be valid for the entire duration of the program
}
 fn main() {

    let p = Person {
        name: "fasterthanlime",
    };
 }