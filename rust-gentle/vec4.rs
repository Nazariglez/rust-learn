fn main() {
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    let v2 = v1.clone();

    v1.sort();
    println!("v1 {:?}", v1);
    v1.dedup();
    println!("v1 {:?}", v1);
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);

    println!("v2 {:?}", v2);
}