#[macro_export]
macro_rules! res_assert {
    ($expr:expr) => {
        match $expr {
            Ok(_) => {},
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                panic!();
            }
        }
    };
    ($expr:expr, err) => {
        match $expr {
            Ok(a) => {
                eprintln!("❌ Not Error");
                panic!();
            }
            Err(_) => {},
        }
    };
}

