use paq1_lib_error_handler::prelude::{*};

#[test]
pub fn should_combine_error_with_same_status() {

    let result_with_error: ResultErr<()> = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - main".to_string(),
        code: "errbadrequest - main".to_string(),
        status: 400,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ A manquant".to_string(), description: None, warn_message: None }]
    }));

    let result_with_other_error_1: ResultErr<()> = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - other 1".to_string(),
        code: "errbadrequest - other 1".to_string(),
        status: 400,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ B manquant".to_string(), description: None, warn_message: None }]
    }));

    let result_with_other_error_2: ResultErr<()> = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - other 2".to_string(),
        code: "errbadrequest - other 2".to_string(),
        status: 400,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ C manquant".to_string(), description: None, warn_message: None }]
    }));

    let combine_error = result_with_error.combine(&result_with_other_error_1).combine(&result_with_other_error_2);

    match combine_error {
        Err(Error::ErrorWithCode(error)) => {
            assert_eq!(error.problems.len(), 5);
            assert_eq!(error.title, "bad request - main");

            let pb1 = error.problems.get(0).unwrap();
            let pb2 = error.problems.get(1).unwrap();
            let pb3 = error.problems.get(2).unwrap();
            let pb4 = error.problems.get(3).unwrap();
            let pb5 = error.problems.get(4).unwrap();

            assert_eq!(pb1.title, "champ A manquant");
            assert_eq!(pb2.title, "[errbadrequest - other 1] - bad request - other 1");
            assert_eq!(pb3.title, "champ B manquant");
            assert_eq!(pb4.title, "[errbadrequest - other 2] - bad request - other 2");
            assert_eq!(pb5.title, "champ C manquant");
        },
        Ok(_) => panic!("Should have been an ErrorWithCode returned"),
    }

}

#[test]
fn should_mapping_vec_of_result_err_to_result_err_vec_when_contain_only_ok_statement_test() {
    let datas: Vec<ResultErr<i32>> = vec![
        Ok(11),
        Ok(12),
        Ok(13)
    ];

    let flatten_result: ResultErr<Vec<i32>> = datas.flatten_result_err();

    match flatten_result {
        Ok(vector) => assert_eq!(vector, vec![11, 12, 13]),
        _ => panic!("Should have been an Ok returned"),
    }
}


#[test]
fn should_mapping_vec_of_result_err_to_result_err_vec_when_contain_one_error_test() {

    let result_with_error = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - main".to_string(),
        code: "errbadrequest - main".to_string(),
        status: 400,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ A manquant".to_string(), description: None, warn_message: None }]
    }));

    let datas: Vec<ResultErr<i32>> = vec![
        Ok(11),
        Ok(12),
        result_with_error,
        Ok(13)
    ];

    let flatten_result: ResultErr<Vec<i32>> = datas.flatten_result_err();

    match flatten_result {
        Err(Error::ErrorWithCode(error)) => {
            assert_eq!(error.problems.len(), 1)
        }
        _ => panic!("Should have been an ErrorWithCode returned"),
    }
}

#[test]
fn should_combine_and_mapping_vec_of_result_err_to_result_err_vec_when_contain_many_error_test() {

    let result_with_error = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - main".to_string(),
        code: "errbadrequest - main".to_string(),
        status: 400,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ A manquant".to_string(), description: None, warn_message: None }]
    }));

    let result_with_other_error_1 = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - other 1".to_string(),
        code: "errbadrequest - other 1".to_string(),
        status: 400,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ B manquant".to_string(), description: None, warn_message: None }]
    }));

    let datas: Vec<ResultErr<i32>> = vec![
        Ok(11),
        Ok(12),
        result_with_error,
        result_with_other_error_1,
        Ok(13)
    ];

    let flatten_result: ResultErr<Vec<i32>> = datas.flatten_result_err();

    match flatten_result {
        Err(Error::ErrorWithCode(error)) => {
            assert_eq!(error.problems.len(), 3)
        }
        _ => panic!("Should have been an ErrorWithCode returned"),
    }
}

#[test]
fn should_add_warn_in_error_when_combine_different_status_test() {

    let result_with_error = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - main".to_string(),
        code: "errbadrequest - main".to_string(),
        status: 400,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ A manquant".to_string(), description: None, warn_message: None }]
    }));

    let result_with_other_error_1 = Err(Error::ErrorWithCode(ErrorWithCode {
        title: "bad request - other 1".to_string(),
        code: "errbadrequest - other 1".to_string(),
        status: 404,
        description: Some("la commande est invalide".to_string()),
        problems: vec![Problem { title: "champ B manquant".to_string(), description: None, warn_message: None }]
    }));

    let datas: Vec<ResultErr<i32>> = vec![
        Ok(11),
        Ok(12),
        result_with_error,
        result_with_other_error_1,
        Ok(13)
    ];

    let flatten_result: ResultErr<Vec<i32>> = datas.flatten_result_err();
    assert!(true);
    match flatten_result {
        Err(Error::ErrorWithCode(error)) => {
            println!("{error:?}");
            assert_eq!(error.problems.get(0).unwrap().warn_message, None);
            assert_eq!(error.problems.get(1).unwrap().warn_message, Some("warn: status conflict from original error, maybe take care about this.".to_string()));
            assert_eq!(error.problems.get(2).unwrap().warn_message, None);
        }
        _ => panic!("Should have been an ErrorWithCode returned"),
    }
}


#[test]
fn should_combine_void_test() {

    let a = ();
    let b = ();

    let res = a.combine(&b);
    assert_eq!(res, ());
}
