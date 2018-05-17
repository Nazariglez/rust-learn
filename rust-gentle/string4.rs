fn main() {
    //version 1
    let text = "the red fox and the lazy dog";
    let stripped:String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();

    let words: Vec<&str> = text.split_whitespace().collect();

    println!("{}",stripped);
    println!("{:?}",words);

    //version 2 using loop
    let mut stripped2:String = "".to_string();
    for ch in text.chars() {
        if !ch.is_whitespace() {
            stripped2 += &ch.to_string();
        }
    }

    println!("{}", stripped2);
}