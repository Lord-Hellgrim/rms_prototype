

#[derive(Debug)]
pub enum RmsError {
    Format,
    Mysql(mysql::Error),
}

impl From<mysql::Error> for RmsError{
    fn from(e: mysql::Error) -> Self {
        RmsError::Mysql(e)
    }
}