use actix_web::{web, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{pg::PgConnection, r2d2::PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct CoreDBPool(pub PgPool);

impl CoreDBPool {
    // Create db connection pool for core database
    pub fn default() -> CoreDBPool {
        let url = std::env::var("DATABASE_URL").unwrap();
        CoreDBPool(init_pool(url.as_str()).expect("Failed to create pool for core DB"))
    }
}
pub trait DBPoolConvertable {
    fn to_pgpool(&self) -> &PgPool;
}

impl DBPoolConvertable for CoreDBPool {
    fn to_pgpool(&self) -> &PgPool {
        &self.0
    }
}

// Convert data of pool type to pgconnection
pub fn pgdata_to_pgconnection<T: DBPoolConvertable>(pgdata: web::Data<T>) -> PgPooledConnection {
    pgdata
        .to_pgpool()
        .get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
        .expect("Getting pg connection exception")
}

// Initiate pgpool from the database in database_url
pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_pool_from_str_failed() -> Result<(), ()> {
        let pool = init_pool("NotCorrect");
        match pool {
            Err(_) => Ok(()),
            _ => Err(()),
        }
    }
    #[test]
    fn init_pool_from_str_succ() -> Result<(), ()> {
        let url = std::env::var("DATABASE_URL").unwrap();
        let pool = init_pool(url.as_str());
        match pool {
            Ok(_) => Ok(()),
            _ => Err(()),
        }
    }
}
