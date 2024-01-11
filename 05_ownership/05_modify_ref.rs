fn main(){
    let mut s = String::from("hello");
    change(&mut s);                         // mutable reference

    println!("{}", s);
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}
