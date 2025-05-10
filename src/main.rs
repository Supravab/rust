fn main() {
    println!("Hello, world!"); //here ! is a macro
    let a=10;
    let b=20;
    if a<b {
        println!("{} is the smallest", a);
    }
    else {
        println!("{} is the smallest",b);
    }
}
//what is a macro?
// macro is basically a function that can take in variable number of arguements, it can check syntax before running.
