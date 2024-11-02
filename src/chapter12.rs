#[test]
fn test1211(){
    fn main(){
        let decimal = 97.123_f32;

        let integer: u8 = decimal as u8; // Specify the type as `u8`

        let c1: char = integer as char; // Changed casting from `decimal as char` to `integer as char`
        let c2 = integer as char;

        assert_eq!(integer, 'a' as u8); // Correct the assertion; 'a' is 97 in ASCII

        println!("Success!");
    }
    main()
}

#[test]
fn test1212(){
    fn main(){
        assert_eq!(u8::MAX, 255);
        // The max of `u8` is 255 as shown above.
        // so the below code will cause an overflow error: literal out of range for `u8`.
        // PLEASE looking for clues within compile errors to FIX it.
        // DON'T modify any code in main.
        let v = 255 as u8; // Replace 1000 with 255 or another valid `u8` value.

        println!("Success!");
    }
    main()
}

#[test]
fn test1213(){
    fn main(){
        assert_eq!(1000 as u16, 1000);

        assert_eq!(1000 % 256, 232);

        println!("1000 mod 256 is : {}", 1000 % 256);

        assert_eq!(-1_i8 as u8, 255);

        assert_eq!(300.1_f32 as u8, 255);
        assert_eq!(-100.1_f32 as u8, 0);

        unsafe {
            println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
            println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
            println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
        }
    }
    main()
}

#[test]
fn test1214(){
    fn main(){
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address: usize = p1 as usize;
        let second_address = first_address + 4;
        let p2: *mut i32 = second_address as *mut i32;
        unsafe {
            *p2 += 1;
        }

        assert_eq!(values[1], 3);

        println!("Success!");
    }
    main()
}

#[test]
fn test1215(){
    fn main(){
        let arr: [u64; 13] = [0; 13];
        assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
        let a: *const [u64; 13] = &arr;
        let b = a as *const [u8; 104]; // 104 = 8 * 13
        unsafe {
            assert_eq!(std::mem::size_of_val(&*b), 104)
        }

        println!("Success!");
    }
    main()
}

#[test]
fn test1221(){
    fn main(){
        // impl From<bool> for i32
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        // FIX the error in two ways
        let i3: i32 = 'a' as i32;

        // FIX the error in two ways
        let s: String = 'a'.to_string();

        println!("Success!");
    }
    main()
}

#[test]
fn test1222(){
    fn main(){
        // impl From<bool> for i32
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        // FIX the error in two ways
        let i3: i32 = 'a' as i32;

        // FIX the error in two ways
        let s: String = 'a'.to_string();

        println!("Success!");
    }
    main()
}

#[test]
fn test1223(){
    use std::fs;
    use std::io;
    use std::num;

    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError {
            CliError::IoError(err) // Перетворення io::Error у CliError
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError {
            CliError::ParseError(err) // Перетворення ParseIntError у CliError
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        // ? автоматично перетворює io::Error у CliError
        let contents = fs::read_to_string(&file_name)?;
        // num::ParseIntError -> CliError
        let num: i32 = contents.trim().parse()?;
        Ok(num)
    }
    fn main(){
        match open_and_parse_file("file.txt") {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => match e {
                CliError::IoError(err) => eprintln!("I/O error: {}", err),
                CliError::ParseError(err) => eprintln!("Parse error: {}", err),
            },
        }
    }
    main();
}

#[test]
fn test1225(){
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(())
            }
        }
    }

    fn main(){
        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(EvenNum::try_from(5), Err(()));

        let result: Result<EvenNum, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));
        let result: Result<EvenNum, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));

        println!("Success!");
    }
    main()
}

#[test]
fn test1231(){
    use std::fmt;

    struct Point1 {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point1 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

    fn main(){
        let origin = Point1 { x: 0, y: 0 };

        assert_eq!(origin.to_string(), "The point is (0, 0)");
        assert_eq!(format!("{}", origin), "The point is (0, 0)");

        println!("Success! 1");
    }
    main()
}

#[test]
fn test1232(){
    use std::str::FromStr;

    fn main() {
        let parsed: i32 = "5".parse().unwrap(); // Use the `parse` method to convert the string to an i32
        let turbo_parsed: i32 = "10".parse().unwrap(); // Similarly, use `parse` for this value
        let from_str = i32::from_str("20").unwrap(); // Use `i32::from_str` explicitly

        let sum = parsed + turbo_parsed + from_str;
        assert_eq!(sum, 35);

        println!("Success!");
    }
    main()
}

#[test]
fn test1233(){
    use std::str::FromStr;
    use std::num::ParseIntError;

    #[derive(Debug, PartialEq)]
    struct Point3 {
        x: i32,
        y: i32,
    }

    impl FromStr for Point3 {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .map(|x| x.trim())
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point3 { x: x_fromstr, y: y_fromstr })
        }
    }

    fn main(){
        let p = Point3::from_str("(3, 4)"); // Перший спосіб

        assert_eq!(p.unwrap(), Point3 { x: 3, y: 4 });
        println!("Success!");
    }
    main()
}