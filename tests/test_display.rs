use x11oo::Display;

#[test]
fn test_debug() {
    match Display::open(None::<String>) {
        Ok(display) => println!("{:?}", display),
        Err(err) => {
            panic!("{:?}", err);
        }
    }

    match Display::open(Some(":0")) {
        Ok(display) => println!("{:?}", display),
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
