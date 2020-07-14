/*!

Redis Hashes are maps between the string fields and the string values. Hence, they are the perfect data type to represent objects.

*/

pub struct Hashes {
  con: redis::aio::Connection,
}

impl Hashes {
  pub fn new(con: redis::aio::Connection) -> Self {
    Hashes { con }
  }

  /**
  Redis HSET command is used to set field in the hash stored at the key to value. If the key does not exist, a new key holding a hash is created. If the field already exists in the hash, it is overwritten.

  ```
  async fn should_hset(redis_hashes: &mut hashes::Hashes) {
    redis_hashes.hset("hash-key", vec!["key1", "value1"]).await;
    let get_result1 = redis_hashes.hget("hash-key", "key1").await;

    assert_eq!(get_result1, "value1");
  }

  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::hashes;

  #[tokio::main]
  async fn tests() -> Result<(), ()> {
    let con_result = connection::connect("redis://127.0.0.1/").await;

    match con_result {
      Ok(mut con) => {
        should_hset(&mut hashes::Hashes::new(con)).await;

        Ok(())
      },
      Err(_) => panic!("can not connect to db"),
    }
  }

  tests();
  ```
  */

  pub async fn hset(&mut self, key: &str, values: Vec<&str>) -> Vec<String> {
    let result = redis::cmd("HSET")
      .arg(key)
      .arg(values)
      .query_async::<redis::aio::Connection, Vec<String>>(&mut self.con)
      .await;
    match result {
      Ok(value) => value,
      Err(_) => vec!["".to_string()],
    }
  }

  /**
  Redis HGET command is used to get the value associated with the field in the hash stored at the key.

  ```
  async fn should_hget(redis_hashes: &mut hashes::Hashes) {
    redis_hashes.hset("hash-key", vec!["key1", "value1"]).await;
    let get_result1 = redis_hashes.hget("hash-key", "key1").await;

    assert_eq!(get_result1, "value1");
  }

  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::hashes;

  #[tokio::main]
  async fn tests() -> Result<(), ()> {
    let con_result = connection::connect("redis://127.0.0.1/").await;

    match con_result {
      Ok(mut con) => {
        should_hget(&mut hashes::Hashes::new(con)).await;

        Ok(())
      },
      Err(_) => panic!("can not connect to db"),
    }
  }

  tests();
  ```
  */

  pub async fn hget(&mut self, key: &str, field: &str) -> String {
    let result = redis::cmd("HGET")
      .arg(key)
      .arg(field)
      .query_async::<redis::aio::Connection, String>(&mut self.con)
      .await;
    match result {
      Ok(value) => value,
      Err(_) => "".to_string(),
    }
  }
}
