use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}

#[derive(Debug, Error)]
enum DatabaseError{
    #[error("error querying database")]
    QueryFailure,
}

#[derive(Debug, Error)]
enum ApiError{
    #[error("network error: {0}")]
    Network(#[from] NetworkError), //can also use from macro provided by thiserror crate
    #[error("network error: {0}")]
    Database(#[from] DatabaseError), // instead of impl manually
}

fn do_stuff() -> Result<(), ApiError>{
    Err(DatabaseError::QueryFailure)?
}