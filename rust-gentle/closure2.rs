fn apply<F: Fn(f64) -> f64> (x: f64, f: F) -> f64 {
    f(x)
}

fn main () {
    let m = 2.0;
    let c = 1.0;

    let lin = |x| m*x+c;

    println!("res {} {}", lin(1.0), lin(2.0));

    let res1 = apply(3.0, lin);
    println!("res {}",res1);

    let res2 = apply(3.14, |x| x.sin());
    let l = lin;

    println!("res {}",res2);
}