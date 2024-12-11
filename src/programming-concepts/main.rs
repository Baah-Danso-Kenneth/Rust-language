// variables and re -assign
// fn main() {
//     let mut x=5;
//     println!("The value of x is: {x}");
//     x=32;
//     print!("The value of x is : {}",x);
// }


// Shadowing

fn main(){
    let x=5;
    print!("This is the value of x: {}",x);

    let x=x +5;
    print!("This is also the value of x: {}",x);
}
