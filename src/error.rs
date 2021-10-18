








use toql::error::ToqlError;
use toql_mysql_async::error::ToqlMySqlAsyncError;
use rocket::{Request, Response, http::Status};
use std::io::Cursor;

use toql_rocket::bad_request_template;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Debug)]
pub struct ServerError (ToqlMySqlAsyncError);

impl From<ToqlMySqlAsyncError> for ServerError{
        fn from(err: ToqlMySqlAsyncError) -> ServerError {
            ServerError(err)
    }
}

impl From<ToqlError> for ServerError{
        fn from(err: ToqlError) -> ServerError {
            ServerError(ToqlMySqlAsyncError::ToqlError(err))
    }
}


impl<'r> rocket::response::Responder<'r, 'static> for ServerError {

    fn respond_to(self, request: &'r Request<'_>) -> std::result::Result<Response<'static>, Status> {
        let mut response = Response::new();
      
        match self.0 {
            ToqlMySqlAsyncError::ToqlError(err) => {
                // Toql produces different status codes
                // Because of orphan rule we have to wrap it
                toql_rocket::error::ToqlErrorWrapper(err).respond_to(request)
                
            }
            ToqlMySqlAsyncError::MySqlError(err) => {
               response.set_status(Status::BadRequest);
               let msg = bad_request_template!(err);
               response.set_sized_body(msg.len(), Cursor::new(msg));
               Ok(response)
            }
            ToqlMySqlAsyncError::FromValueError(err) => {
               response.set_status(Status::BadRequest);
               let msg = bad_request_template!(err);
               response.set_sized_body(msg.len(), Cursor::new(msg));
               Ok(response)
            }
            
        }

    }
}


