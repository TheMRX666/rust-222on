#[test]
fn test1311() {
    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            println!("Success!");
        } else {
            panic!("This is not lemonade!");
        }
        println!("Exercise Failed if printing out this line!");
    }
        drink("lemonade");
        println!("Exercise Failed if printing out this line!");
}

#[test]
fn test1312() {
        assert_eq!("abc".as_bytes(), [97, 98, 99]);
        let v = vec![1, 2, 3];
        if let Some(ele) = v.get(3) {
            println!("Found element: {}", ele);
        } else {
            println!("Element not found!");
        }
        let v = production_rate_per_hour(2);
        println!("Production rate: {}", v);

        divide(15, 0);

        println!("Success!");
    fn divide(x:u8, y:u8) {
        if y == 0 {
            println!("Cannot divide by zero");
        } else {
            println!("{}", x / y);
        }
    }
    fn production_rate_per_hour(speed: u8) -> f64 {
        let cph: u8 = 221;
        match speed {
            1..=4 => speed.wrapping_mul(cph)  as f64,
            5..=8 => speed.wrapping_mul(cph)  as f64 * 0.9,
            9..=10 => speed.wrapping_mul(cph)  as f64 * 0.77,
            _ => 0.0,
        }
    }
    pub fn working_items_per_minute(speed: u8) -> u32 {
        (production_rate_per_hour(speed) / 60.0) as u32
    }
}

#[test]
fn test1313() {
    // $ RUST_BACKTRACE=full cargo run
}

#[test]
fn test1321() {
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));

        let result = multiply("t", "2");
        assert_eq!(result.is_err(), true);

        println!("Success!");
}

#[test]
fn test1322() {
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>  {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }
        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!");
}

#[test]
fn test1323() {

    use std::fs::File;
    use std::io::{self, Read};

    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
        assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
        println!("Success!");
}

#[test]
fn test1324() {
    use std::num::ParseIntError;

    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        n_str.parse::<i32>().map(|n| n + 2)
    }
        assert_eq!(add_two("4").unwrap(), 6);

        println!("Success!");
}

#[test]
fn test1325() {
    use std::num::ParseIntError;
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        match n1_str.parse::<i32>() {
            Ok(n1)  => {
                match n2_str.parse::<i32>() {
                    Ok(n2)  => {
                        Ok(n1 * n2)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }
    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str.parse::<i32>().and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
    }
    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
        let twenty = multiply1("10", "2");
        print(twenty);

        let tt = multiply("t", "2");
        print(tt);

        println!("Success!");
}

#[test]
fn test1326() {
    use std::num::ParseIntError;
    type Res<T> = Result<T, ParseIntError>;
    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }
    fn print(result: Res<i32>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
     print(multiply("10", "2"));
        print(multiply("t", "2"));

        println!("Success!");
}