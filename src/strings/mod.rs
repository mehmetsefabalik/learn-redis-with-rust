/*!

Redis strings commands are used for managing string values in Redis.

*/


pub struct Strings {
  con: redis::aio::Connection,
}

impl Strings {
  pub fn new(con: redis::aio::Connection) -> Self {
    Strings {
      con
    }
  }

  /**
  Sets string

  ```
  async fn should_set(redis_strings: &mut strings::Strings) {
    redis_strings.del("excellent-key").await;
    redis_strings.set("excellent-key", "my-value").await;
    let get_result = redis_strings.get("excellent-key").await;

    assert_eq!(get_result, "my-value");
  }

  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::strings;

  #[tokio::main]
  async fn tests() -> Result<(), ()> {
    let con_result = connection::connect("redis://127.0.0.1/").await;

    match con_result {
      Ok(mut con) => {
        should_set(&mut strings::Strings::new(con)).await;

        Ok(())
      },
      Err(_) => panic!("can not connect to db"),
    }
  }

  tests();
  ```
  */

  pub async fn set(&mut self, key: &str, value: &str) -> u8 {
    let result = redis::cmd("SET").arg(key).arg(value).query_async::<redis::aio::Connection, u8>(&mut self.con).await;
    match result {
      Ok(value) => value,
      Err(_) => 0
    }
  }

  /**
  Gets string

  ```
  async fn should_get(redis_strings: &mut strings::Strings) {
    redis_strings.del("excellent-key").await;
    redis_strings.set("excellent-key", "my-value").await;
    let get_result = redis_strings.get("excellent-key").await;

    assert_eq!(get_result, "my-value");
  }

  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::strings;

  #[tokio::main]
  async fn tests() -> Result<(), ()> {
    let con_result = connection::connect("redis://127.0.0.1/").await;

    match con_result {
      Ok(mut con) => {
        should_get(&mut strings::Strings::new(con)).await;

        Ok(())
      },
      Err(_) => panic!("can not connect to db"),
    }
  }

  tests();
  ```
  */

  pub async fn get(&mut self, key: &str) -> String {
    let result = redis::cmd("GET").arg(key).query_async::<redis::aio::Connection, String>(&mut self.con).await;
    match result {
      Ok(value) => value,
      Err(_) => "".to_string()
    }
  }

  /**
  Deletes string
  ```
  async fn should_delete(redis_strings: &mut strings::Strings) {
    redis_strings.del("excellent-key").await;
    let get_result = redis_strings.get("excellent-key").await;

    assert_eq!(get_result, "");
  }

  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::strings;

  #[tokio::main]
  async fn tests() -> Result<(), ()> {
    let con_result = connection::connect("redis://127.0.0.1/").await;

    match con_result {
      Ok(mut con) => {
        should_delete(&mut strings::Strings::new(con)).await;

        Ok(())
      },
      Err(_) => panic!("can not connect to db"),
    }
  }

  tests();
  ```
  */

  pub async fn del(&mut self, key: &str) -> String {
    let result = redis::cmd("DEL").arg(key).query_async::<redis::aio::Connection, String>(&mut self.con).await;
    match result {
      Ok(value) => value,
      Err(_) => "".to_string()
    }
  }

  /**
  used to get the substring of the string value stored at the key,
  determined by the offsets start and end (both are inclusive).
  Negative offsets can be used in order to provide an offset starting from the end of the string.
  ```
  async fn shoult_get_range(redis_strings: &mut strings::Strings) {
    redis_strings.set("key44", "this is me").await;
    let get_result = redis_strings.get_range("key44", 0, 3).await;
    let get_result_all = redis_strings.get_range("key44", 0, -1).await;

    assert_eq!(get_result_all, "this is me");
  }

  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::strings;

  #[tokio::main]
  async fn tests() -> Result<(), ()> {
    let con_result = connection::connect("redis://127.0.0.1/").await;

    match con_result {
      Ok(mut con) => {
        shoult_get_range(&mut strings::Strings::new(con)).await;

        Ok(())
      },
      Err(_) => panic!("can not connect to db"),
    }
  }

  tests();
  ```
  */

  pub async fn get_range(&mut self, key: &str, start: u8, end: i8) -> String {
    let result = redis::cmd("GETRANGE").arg(key).arg(start).arg(end).query_async::<redis::aio::Connection, String>(&mut self.con).await;
    match result {
      Ok(value) => value,
      Err(_) => "".to_string()
    }
  }

  /**
  sets the specified string value in Redis key and returns its old value.
  ```
  async fn shoult_get_set(redis_strings: &mut strings::Strings) {
    redis_strings.set("key44", "this is me").await;
    let get_set_result = redis_strings.get_set("key44", "this is set").await;

    assert_eq!(get_set_result, "this is me");

    let get_result = redis_strings.get("key44").await;

    assert_eq!(get_result, "this is set");
  }

  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::strings;

  #[tokio::main]
  async fn tests() -> Result<(), ()> {
    let con_result = connection::connect("redis://127.0.0.1/").await;

    match con_result {
      Ok(mut con) => {
        shoult_get_set(&mut strings::Strings::new(con)).await;

        Ok(())
      },
      Err(_) => panic!("can not connect to db"),
    }
  }

  tests();
  ```
  */

  pub async fn get_set(&mut self, key: &str, value: &str) -> String {
    let result = redis::cmd("GETSET").arg(key).arg(value).query_async::<redis::aio::Connection, String>(&mut self.con).await;
    match result {
      Ok(value) => value,
      Err(_) => "".to_string()
    }
  }
}
