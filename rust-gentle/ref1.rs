fn main() {
    let s1 = "hello dolly".to_string();
    let mut rs1 = &s1;

    {
        let tmp = "hello world".to_string();
        rs1 = &tmp;
    }

    println!("ref {}", rs1);
}

//fails because tmp is dead outside of his block, so the reference to rs1 is already dead when the println use it