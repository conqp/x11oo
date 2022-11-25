use x11oo::Display;

#[test]
fn test_debug() {
    let display = Display::open(None::<String>);
    assert!(display.is_some());
    println!("{:?}", display.unwrap());

    let display = Display::open(Some(":0"));
    assert!(display.is_some());
    println!("{:?}", display.unwrap());
}
