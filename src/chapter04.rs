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