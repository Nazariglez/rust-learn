#[derive(Debug)]
struct A {
    s: &str
}

fn main() { 
    let a = A { s: "hello dammit" };
    println!("{:?}", a);
}

//fails because the struct don't know the lifetime of the reference to &str 