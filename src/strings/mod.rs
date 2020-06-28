/*!

Redis strings commands are used for managing string values in Redis.

*/

/**
Sets string

```
async fn should_set(con: &mut redis::aio::Connection) {
  strings::del("excellent-key", con).await;
  strings::set("excellent-key", "my-value", con).await;
  let get_result = strings::get("excellent-key", con).await;

  assert_eq!(get_result, "my-value");
}

use learn_redis_with_rust::connection;
use learn_redis_with_rust::strings;

#[tokio::main]
async fn tests() -> Result<(), ()> {
  let con_result = connection::connect("redis://127.0.0.1/").await;

  match con_result {
    Ok(mut con) => {
      should_set(&mut con).await;

      Ok(())
    },
    Err(_) => panic!("can not connect to db"),
  }
}

tests();
```
*/

pub async fn set(key: &str, value: &str, con: &mut redis::aio::Connection) -> u8 {
  let result = redis::cmd("SET").arg(key).arg(value).query_async::<redis::aio::Connection, u8>(con).await;
  match result {
    Ok(value) => value,
    Err(_) => 0
  }
}

/**
Gets string

```
async fn should_get(con: &mut redis::aio::Connection) {
  strings::del("excellent-key", con).await;
  strings::set("excellent-key", "my-value", con).await;
  let get_result = strings::get("excellent-key", con).await;

  assert_eq!(get_result, "my-value");
}

use learn_redis_with_rust::connection;
use learn_redis_with_rust::strings;

#[tokio::main]
async fn tests() -> Result<(), ()> {
  let con_result = connection::connect("redis://127.0.0.1/").await;

  match con_result {
    Ok(mut con) => {
      should_get(&mut con).await;

      Ok(())
    },
    Err(_) => panic!("can not connect to db"),
  }
}

tests();
```
*/

pub async fn get(key: &str, con: &mut redis::aio::Connection) -> String {
  let result = redis::cmd("GET").arg(key).query_async::<redis::aio::Connection, String>(con).await;
  match result {
    Ok(value) => value,
    Err(_) => "".to_string()
  }
}

/**
Deletes string
```
async fn should_delete(con: &mut redis::aio::Connection) {
  strings::del("excellent-key", con).await;
  let get_result = strings::get("excellent-key", con).await;

  assert_eq!(get_result, "");
}

use learn_redis_with_rust::connection;
use learn_redis_with_rust::strings;

#[tokio::main]
async fn tests() -> Result<(), ()> {
  let con_result = connection::connect("redis://127.0.0.1/").await;

  match con_result {
    Ok(mut con) => {
      should_delete(&mut con).await;

      Ok(())
    },
    Err(_) => panic!("can not connect to db"),
  }
}

tests();
```
*/

pub async fn del(key: &str, con: &mut redis::aio::Connection) -> String {
  let result = redis::cmd("DEL").arg(key).query_async::<redis::aio::Connection, String>(con).await;
  match result {
    Ok(value) => value,
    Err(_) => "".to_string()
  }
}