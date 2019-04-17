// all read_line
// default i32
// let t = read!()
// let t = read!(f64)
// read a Vec<type> into t
// read!(type, t)
#[macro_export]
macro_rules! read {
    () => {{
        let mut buffer = std::string::String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("{:?}", err),
            Ok(_) => buffer.trim().parse::<i32>().unwrap(),
        }
    }};
    ($t:ty) => {{
        let mut buffer = std::string::String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("{:?}", err),
            Ok(_) => buffer.trim().parse::<$t>().unwrap(),
        }
    }};
    ($t:ty, $e:expr) => {{
        let mut buffer = std::string::String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("{:?}", err),
            Ok(_) => {
                $e = buffer
                    .split_whitespace()
                    .map(|s| s.parse::<$t>().unwrap())
                    .collect()
            }
        }
    }};
}

#[macro_export]
macro_rules! cin {
    () => {};
    ($e0:expr) => {
        let stdin = std::io::stdin();
        $e0 = String::from_utf8(
            std::io::Read::bytes(stdin.lock())
                .map(|c| c.unwrap())
                .skip_while(|c| c.is_ascii_whitespace())
                .take_while(|c| !c.is_ascii_whitespace())
                .collect())
            .expect("string")
            .parse().expect("valid parse");
    };
    ($e0:expr, $($ei:tt)*) => {
        let stdin = std::io::stdin();
        $e0 = String::from_utf8(
            std::io::Read::bytes(stdin.lock())
                .map(|c| c.unwrap())
                .skip_while(|c| c.is_ascii_whitespace())
                .take_while(|c| !c.is_ascii_whitespace())
                .collect())
            .expect("string")
            .parse().expect("valid parse");
        cin!($($ei)*);
    };
}

