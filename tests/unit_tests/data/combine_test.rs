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