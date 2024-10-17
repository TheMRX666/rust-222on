use std::mem::size_of;
use std::mem::size_of_val;

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


#[test]
fn test411() {
    let x = 5;
    let mut y = 5;
    y = x;
    let _z = 10; // Type of z ?

    println!("Success!");
}

#[test]
fn test412() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
fn test413() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

#[test]
fn test414() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

#[test]
fn test415() {
    let v1 = 251_u16 + 8;
    let v2 = u8::checked_add(251, 8).unwrap_or(0);
    println!("{},{}", v1, v2);
}

#[test]
fn test416() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

#[test]
fn test417() {
    let x = 1_000.000_1;
    let y: f32 = 0.12;
    let z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

#[test]
fn test418() {
    assert!((0.1+0.2-0.3) < f64::EPSILON);
    println!("Success!");
}

#[test]
fn test419() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

#[test]
fn test421() {
    let c1 = 'a';
    assert_eq!(size_of::<char>(), 4);

    let c2 = '中';
    assert_eq!(size_of::<char>(), 4);

    println!("Success!");
}

#[test]
fn test422() {
    fn main() {
        let c1 = "中";
        print_char(c1);
    }

    fn print_char(c: &str) {
        println!("{}", c);
    }
}

#[test]
fn test423() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test424() {
    let f = true;
    let t = true;
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test425() {
    fn main() {
        let _v: () = ();

        let v = (2, 3);
        assert_eq!(v, explicitly_ret_unit());

        println!("Success!");
    }

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

    // Don't use this one
    fn explicitly_ret_unit() -> (i32, i32) {
        println!("I will return a ()");
        (2, 3)
    }
}

#[test]
fn test426() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 1 || size_of_val(&unit) == 0 || size_of_val(&unit) == 4);

    println!("Success!");
}

#[test]
fn test431() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

#[test]
fn test432() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);
    println!("Success!");
}

#[test]
fn test433() {
    fn main() {
        let s = sum(1 , 2);
        assert_eq!(s, 3);

        println!("Success!");
    }
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
}