use etcd_client::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 连接etcd
    let mut client = Client::connect(["127.0.0.1:2379"], None).await?;
    // 设置key
    client.put("hello", "world", None).await?;
    // 获取key
    let resp = client.get("hello", None).await?;
    if let Some(kv) = resp.kvs().first() {
        println!("value is {:?}", kv.value_str().unwrap());
    }

    Ok(())
}
