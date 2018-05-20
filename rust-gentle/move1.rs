fn main() {
    let s1 = "hello dolly".to_string(); 
    let s2 = s1;
    println!("st {}", s1);    
}

//fails because s1 has been moved to s2