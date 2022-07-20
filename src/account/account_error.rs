#[derive(Debug, thiserror::Error)]
pub enum AccountError {}

// impl ErrorExtensions for AccountError {
//     fn extend(&self) -> Error {
//         Error::new(format!("{}", self)).extend_with(|err, e| match self {
//             MyError::NotFound => e.set("code", "NOT_FOUND"),
//             MyError::ServerError(reason) => e.set("reason", reason.clone()),
//             MyError::ErrorWithoutExtensions => {}
//         })
//     }
// }
