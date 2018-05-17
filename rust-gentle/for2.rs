fn main(){
    for i in 0..5 {
        if i % 2 == 2 {
            println!("Even {}", i);
        } else {
            println!("Odd {}", i);
        }
    }
}