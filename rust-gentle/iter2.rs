fn main() {
    let arr = [10, 20, 30];
    for i in arr {
        println!("{}", i);
    }
}

//fails because arr needs to use .iter()