fn sqr<T> (x: T) -> T::Output 
where T: std::ops::Mul + Copy {
    x * x
}

fn main() {
    let res = sqr(10.0);
    println!("res {}", res);
}