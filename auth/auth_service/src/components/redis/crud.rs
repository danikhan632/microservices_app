use redis::{Client, Connection, Commands};

fn create(conn: &mut Connection, key: &str, value: &str) -> redis::RedisResult<()> {
    conn.set(key, value)
}

fn read(conn: &mut Connection, key: &str) -> redis::RedisResult<String> {
    conn.get(key)
}

fn update(conn: &mut Connection, key: &str, value: &str) -> redis::RedisResult<()> {
    create(conn, key, value)
}

fn delete(conn: &mut Connection, key: &str) -> redis::RedisResult<()> {
    conn.del(key)
}



// fn main() {
//     let client = Client::open("redis://127.0.0.1/").unwrap();
//     let mut conn = client.get_connection().unwrap();

//     create(&mut conn, "key", "value").unwrap();
//     println!("{}", read(&mut conn, "key").unwrap());
//     update(&mut conn, "key", "new_value").unwrap();
//     println!("{}", read(&mut conn, "key").unwrap());
//     delete(&mut conn, "key").unwrap();
//     println!("{}", read(&mut conn, "key").unwrap());
// }
