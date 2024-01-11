fn main() {
    let s = String::from("hello");

    takes_ownership(s);                 // s's value moves into the fun..
                                        // so is no longer valid here
    //println!("{}", s);                // no ownership 

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}   
// some_string goes out of scope and `drop` is called. 
// The backing memory is freed.


fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
// Here, some_integer goes out of scope. Nothing special happens.

