pub fn str_as_bulk_str(orig: &str) -> String {
    kv_as_bulk_str(&vec![orig])
}

pub fn kv_as_bulk_str(kv: &Vec<&str>) -> String {
    let kv: Vec<String> = kv.chunks(2).map(|chunk| chunk.to_vec().join(":")).collect();
    let payload = kv.join("\r\n");
    format!("${}\r\n{}\r\n", payload.len(), payload)
}

pub fn vec_w_bulk_strs(strings: &Vec<&str>) -> String {
    let len = strings.len();
    let bulk_strings: Vec<String> = strings.iter().map(|s| str_as_bulk_str(s)).collect();
    format!("*{}\r\n{}", len, bulk_strings.join(""))
}
