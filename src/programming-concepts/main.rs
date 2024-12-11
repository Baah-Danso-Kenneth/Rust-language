// variables and re -assign
// fn main() {
//     let mut x=5;
//     println!("The value of x is: {x}");
//     x=32;
//     print!("The value of x is : {}",x);
// }


// Shadowing

// fn main(){
//     let x=5;
//     print!("This is the value of x: {}",x);

//     let x=x +5;
//     print!("This is also the value of x: {}",x);
// }


// Tupels
// fn main(){
//     let tup = (10,52,89,10.55,"gee");
//     let (_a,_b,_c,_d,_e) = tup;
//     // print!("Print the value of {}",_b);
//     let a=[2,4,6,8,10];
//     println!("{}",tup.2);

// }

// use std::io;

/**Array**/
// fn main(){
//     let a =[2,4,6,10];
//     let mut index = String::new();

//    io::stdin()
//    .read_line(&mut index)
//    .expect("Enter word");

//   let index:usize = index.trim().parse().expect("Please this should be a number");

//   let element = index;
//   print!("Answer has to be {}", a[element]);

// }

fn main(){
    another_function(5);

}

fn another_function(x:i32){
    println!("The number of x is : {x}");
    try_func(32, "Kenneth");
}


fn try_func(x:i32,a:&str){
    let age = x;
    let name=a;
  add_something(60);

    println!("My age is {age} and my name is {name}");
}

fn add_something(x:i32) -> i32{
    x * 42

}