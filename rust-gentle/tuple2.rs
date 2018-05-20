fn main() {
    for t in  ["zero", "one", "two"].iter().enumerate() {
        print!(" {} {};", t.0, t.1);
    }

    println!();

    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];
    for p in names.iter().zip(nums.iter()) {
        print!(" {} {};", p.0, p.1);
    }

    println!();
}