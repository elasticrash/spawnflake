/**
 * Create and get a connection
 */
 pub fn create_connection(url: String) -> PooledConn {
    let pool = Pool::new(url);
    return pool.unwrap().get_conn().unwrap();
}