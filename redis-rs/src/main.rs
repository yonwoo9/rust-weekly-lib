use redis::Commands;

extern crate redis;

fn main() -> redis::RedisResult<()> {
    // 连接redis
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    // 设置键值对
    let _: () = con.set("key1", 1).unwrap();
    let _: () = con.set("key2", 2).unwrap();

    // 获取值
    let result: i32 = con.get("key1").unwrap();
    println!("key1: {}", result);

    // 删除键
    let _: i32 = con.del("key2").unwrap();

    // 使用redis列表
    let _: () = con.lpush("mylist", "a").unwrap();
    let _: () = con.lpush("mylist", "b").unwrap();
    let _: () = con.lpush("mylist", "c").unwrap();

    let result: Vec<String> = con.lrange("mylist", 0, -1).unwrap();
    println!("mylist: {:?}", result);

    // 使用redis集合
    let _: () = con.sadd("myset", "a").unwrap();
    let _: () = con.sadd("myset", "b").unwrap();
    let _: () = con.sadd("myset", "c").unwrap();

    let result: Vec<String> = con.smembers("myset").unwrap();
    println!("myset: {:?}", result);

    // 使用redis哈希表
    let _: () = con.hset("myhash", "field1", 1).unwrap();
    let _: () = con.hset("myhash", "field2", 2).unwrap();
    let _: () = con.hset("myhash", "field3", 3).unwrap();

    let result: i32 = con.hget("myhash", "field1").unwrap();
    println!("myhash: {}", result);

    // 使用script
    let script = redis::Script::new(
        r#"
        return redis.call('get', KEYS[1])
    "#,
    );

    let result: i32 = script.key("key1").invoke(&mut con).unwrap();
    println!("script: {}", result);

    Ok(())
}
