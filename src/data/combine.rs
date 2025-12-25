use crate::prelude::{Error, ErrorWithCode, Problem, ResultErr};

pub trait CanCombine {
    fn combine(&self, other: &Self) -> Self;
}

impl CanCombine for Error {
    fn combine(&self, other: &Self) -> Self {
        match (self, other) {
            (Error::Failure(error), Error::Failure(other_error)) => {
                let current_problems: Vec<Problem> = error.problems.clone();
                let is_status_conflict: bool = error.status != other_error.status;

                let maybe_status_conflict_message: Option<String> = if is_status_conflict {
                    Some("warn: status conflict from original error, maybe take care about this.".to_string())
                } else { None };

                let other_problem_message: String = format!("[{}] - {}", other_error.clone().code, other_error.clone().title);

                let first_other_problem = Problem {
                    title: other_problem_message,
                    description: None,
                    warn_message: maybe_status_conflict_message
                };

                let other_problems = [vec![first_other_problem], other_error.problems.clone()].concat();

                let problems: Vec<Problem> = [current_problems, other_problems].concat();

                Error::Failure(
                    ErrorWithCode {
                        problems,
                        ..error.clone()
                    }
                )
            }
        }
    }
}

impl<T> CanCombine for ResultErr<T>
where
    T: CanCombine + Clone,
{
    fn combine(&self, other: &ResultErr<T>) -> Self {

        match (self.clone(), other.clone()) {
            (Ok(a), Ok(b)) => Ok(a.combine(&b)),
            (Err(a), Err(b)) => Err(a.combine(&b)),
            (Ok(_), Err(b)) => Err(b),
            (Err(a), Ok(_)) => Err(a)
        }
    }
}

impl CanCombine for () {
    fn combine(&self, _other: &Self) -> Self {
        ()
    }
}

impl<T> CanCombine for Vec<T>
where
    T: Clone
{
    fn combine(&self, other: &Vec<T>) -> Self {
        [self.clone(), other.clone()].concat()
    }
}


pub trait MergeVecResultErr<T> {
    fn flatten_result_err(&self) -> ResultErr<Vec<T>>;
}

impl<T> MergeVecResultErr<T> for Vec<ResultErr<T>>
where
    T: Clone
{
    fn flatten_result_err(&self) -> ResultErr<Vec<T>> {
        self.iter().fold(Ok(Vec::new()), |acc, current| {
            acc.combine(&current.clone().map(|a| vec![a]))
        })
    }
}
