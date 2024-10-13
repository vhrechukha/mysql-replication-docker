use mysql::prelude::*;
use mysql::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let url = "mysql://root:rootpassword@localhost:3306/testdb";
    let pool = Pool::new(url).expect("Failed to create MySQL connection pool");
    let mut conn = pool.get_conn().expect("Failed to connect to MySQL");

    loop {
        match insert_data(&mut conn) {
            Ok(_) => println!("Data written to master"),
            Err(err) => eprintln!("Failed to write data: {:?}", err),
        }

        sleep(Duration::from_secs(5)).await;
    }
}

fn insert_data(conn: &mut PooledConn) -> Result<()> {
    conn.exec_drop("INSERT INTO test_table (name) VALUES ('Test data')", ())?;
    Ok(())
}
