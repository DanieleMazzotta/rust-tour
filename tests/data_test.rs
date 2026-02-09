use soc_gs::datatypes::Data;

#[cfg(test)]
use pretty_assertions::{assert_eq};

#[test]
fn test_equal() {
    let d = Data {
        id: 1,
        name: String::from("T")
    };
    let dd = Data {
        id: 1,
        name: String::from("T")
    };

    assert_eq!(d, dd);
}