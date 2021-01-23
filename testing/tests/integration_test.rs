use testing;

mod common;

#[test]
fn test_rectangle() {
    common::setup();

    let big_rect = testing::Rectangle {
        width: 10,
        height: 8
    };

    let small_rect = testing::Rectangle {
        width: 5,
        height: 4
    };

    assert!(big_rect.can_hold(&small_rect));
}