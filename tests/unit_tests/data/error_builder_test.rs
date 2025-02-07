use paq1_lib_error_handler::data::error_with_code::{ErrorWithCode, ErrorWithCodeBuilder, Error};
use paq1_lib_error_handler::prelude::Problem;

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
}

#[test]
pub fn should_add_description_to_builder_test() {

    let description_expected = "dEscrIption test";

    let empty_error = ErrorWithCodeBuilder::new("xxx", 400, "test error")
        .with_description(description_expected)
        .build();

    match empty_error {
        Error::ErrorWithCode(ErrorWithCode {code: _, title: _, status: _, description, problems: _}) => {
            assert_eq!(description, Some(description_expected.to_string()));
        }
    }
}


#[test]
pub fn should_add_problems_to_builder_test() {

    let expected_problem = Problem {
        title: "proBlEme".to_string(),
        description: Some("deScrIPtion".to_string()),
        warn_message: Some("WarnMessage".to_string()),
    };

    let empty_error = ErrorWithCodeBuilder::new("xxx", 400, "test error")
        .with_problems(vec![expected_problem.clone()])
        .build();

    match empty_error {
        Error::ErrorWithCode(ErrorWithCode {code: _, title: _, status: _, description: _, problems}) => {
            assert_eq!(problems, vec![expected_problem]);
        }
    }
}

#[test]
pub fn should_format_error_test() {

    let empty_error: Error = ErrorWithCodeBuilder::new("xxx", 400, "test error").build();

    let expected_error_str = "ErrorWithCode { code: \"xxx\", status: 400, title: \"test error\", description: None, problems: [] }";
    let str_error = format!("{empty_error}");

    assert_eq!(expected_error_str, str_error.as_str());
}
