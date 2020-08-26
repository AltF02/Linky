use tokio_postgres::{Client, NoTls, Error, Config};
use r2d2::Pool;

pub(crate) fn init_database() -> Pool<_> {
    let mut cfg = Config::new();
    cfg.host("localhost");
    cfg.user("postgres");
    cfg.password("password123");
    cfg.dbname("postgres");
    cfg.port(5432);
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    Pool::new(mgr, 16)
}