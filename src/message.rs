pub fn kv_as_bulk_str(kv: &Vec<&str>) -> String {
    let kv: Vec<String> = kv.chunks(2).map(|chunk| chunk.to_vec().join(":")).collect();
    let payload = kv.join("\r\n");
    format!("${}\r\n{}", payload.len(), payload)
}

pub fn vec_as_bulk_str(payload: &Vec<String>) -> String {
    let len = payload.len();
    format!("*{}\r\n{}", len, payload.join(""))
}
