fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    let a = [1,2,3];
    let months = ["January", "February", "..."];
    let b: [i32; 5] = [1,2,3,4,5];
    let c = [3; 5];
    println!("c = {}", c[0]);

    another_fun(5);

    for item in a.iter() {
        println!("{}",item);
    }

    for num in (1..4).rev() {
        println!("{}", num);
    }
}


fn another_fun(x: i32){
    println!("another {}", x);
}

