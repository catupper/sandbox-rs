#[derive(Debug)]
pub enum MyError {
    Type { expected: String, got: String },
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Type { expected, got } => write!(f, "Expected {}, got {})", expected, got),
        }
    }
}

impl std::error::Error for MyError {}

#[derive(Debug, failure::Fail)]
pub enum FailureError {
    #[fail(display = "Original Error {}", _0)]
    WrappedError(#[fail(cause)] MyError),
}

#[derive(Debug, thiserror::Error)]
pub enum ThisErrorError {
    #[error("Original Error {0}")]
    WrappedError(#[from] MyError),
}

#[derive(Debug, snafu::Snafu)]
enum SnafuError {
    #[snafu(display("Original Error {}", "source"))]
    WrappedError { source: MyError },
}

fn are_you_error<E: std::error::Error>(_: E) {
    println!("yey~");
}

fn main() {
    let my_error = MyError::Type {
        expected: "Java".to_string(),
        got: "javascript".to_string(),
    };
    are_you_error(my_error);
}
