/*!

Establish connection to Redis Server

*/


/**

```

#[tokio::main]
async fn should_connect_and_do_stuff() -> Result<(), ()> {
  use learn_redis_with_rust::connection;
  use learn_redis_with_rust::strings;

  let con_result = connection::connect("redis://127.0.0.1/").await;

  match con_result {
    Ok(mut con) => {
      let mut redis_strings = strings::Strings::new(con);
      redis_strings.del("excellent-key").await;
      redis_strings.set("excellent-key", "my-value").await;
      let get_result = redis_strings.get("excellent-key").await;

      assert_eq!(get_result, "my-value");

      redis_strings.del("excellent-key").await;
      let get_result = redis_strings.get("excellent-key").await;
      assert_eq!(get_result, "");

      redis_strings.del("excellent-key").await;
      Ok(())
    },
    Err(_) => panic!("can not connect to db"),
  }
}

should_connect_and_do_stuff();

```

*/


pub async fn connect(uri: &str) -> redis::RedisResult<redis::aio::Connection> {
  let client = redis::Client::open(uri).unwrap();
  client.get_async_connection().await
}

