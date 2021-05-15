# rs_json

JSON parser written in Rust

**WARNING: Writing this code was simply an educational exercise for the author. Letting this anywhere near production would be the strongest opposite of a good idea.**

Returns a JsonValue with the following types

```rust
pub enum JsonValue {
    JObj(HashMap<String, JsonValue>),
    JArray(Vec<JsonValue>),
    JString(String),
    JNum(f64),
    JBool(bool),
    JNull,
}
```
