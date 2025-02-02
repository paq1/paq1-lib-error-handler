use paq1_lib_error_handler::data::error_with_code::{ErrorWithCode, ErrorWithCodeBuilder, Error};

#[test]
pub fn should_create_builder_test() {

    let empty_error = ErrorWithCodeBuilder::new("xxx", 400, "test error").build();

    match empty_error {
        Error::ErrorWithCode(ErrorWithCode {code, title, status, description, problems}) => {
            assert_eq!(description, None);
            assert_eq!(title, "test error");
            assert_eq!(status, 400);
            assert_eq!(problems.len(), 0);
            assert_eq!(code, "xxx");
        }
    }

    assert!(true)
}