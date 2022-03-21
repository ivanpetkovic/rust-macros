
use crate::require_feature;


#[require_feature]
struct Student {
    name: String
}

#[test]
fn it_works() {
    let result = Student {
        name: String::from("Ivan")
    };
    assert_eq!(result.n)
}
