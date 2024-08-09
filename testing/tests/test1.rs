/*
Integration test is to test our lib as an external API.
Note, if unit (lib) tests will fail, then integration tests won't be run. */
use testing;

#[test]
fn it_adds_two() {
    assert_eq!(4, testing::add_two(2));
}
