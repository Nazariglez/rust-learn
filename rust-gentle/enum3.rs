#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn eat_and_dump(v: Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}

fn dump(v: &Value) {
    use Value::*;
    match *v {
        Number(n) => println!("number is {}", n),
        Str(ref s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
} 

impl Value {
    fn to_str(self) -> Option<String> {
        match self {
            Value::Str(s) => Some(s),
            _ => None
        }
    }
}

fn main() {
    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);

    println!("n {:?} s {:?} b {:?}", n, s, b);

    dump(&n);
    dump(&s);
    dump(&b);

    println!("s? {:?}", s.to_str());

    println!("Eat and dump");
    eat_and_dump(n);
    //eat_and_dump(s);
    eat_and_dump(b);

    
}