#### 官方仓库
- [github:](https://github.com/Keats/jsonwebtoken) https://github.com/Keats/jsonwebtoken

#### 涉及到的库
- actix-web: Rust Web框架
- actix-web-httpauth：在actix-web应用中实现HTTP身份验证
- jsonwebtoken: 用于JWT的编码和解码
- serde: 用于序列化和反序列化数据
- chrono: 用于处理日期和时间
- derive_more：more derive(Trait) options, 这里用到了`Display` trait

#### 使用
- ``cargo run main.rs``
  
- ``curl -XPOST http://localhost:8080/login -d '{"username": "admin", "password": "password"}' -H 'Content-Type: application/json'
``

- ``
curl http://localhost:8080/protected -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImV4cCI6MTcyMjI1MTU5Nn0.BFv88IDx-FWWH00KbYQiAgISbltxupfcWDbtShWSmto' -H 'Content-Type: application/json'
``