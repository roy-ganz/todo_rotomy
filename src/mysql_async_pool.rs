use rocket::figment::Figment;
use rocket_db_pools::Config;
use std::borrow::Cow;
use toql_mysql_async::mysql_async::{Conn, Error, Pool};

pub struct MySqlAsyncPool(Pool);

#[rocket::async_trait]
impl rocket_db_pools::Pool for MySqlAsyncPool {
    type Connection = Conn;
    type Error = Error;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config: Config = figment
            .extract()
            .map_err(|e| Error::Other(Cow::Owned(e.to_string())))?;

        let pool = MySqlAsyncPool(Pool::from_url(config.url)?);

        Ok(pool)
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        self.0.get_conn().await
    }
}
