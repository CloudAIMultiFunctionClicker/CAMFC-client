fn main() {
    let msg = "你好"; // 创建一个不可变文本
    println!("{}", msg);
    tauri_build::build()
}

fn get_id() -> String {
    return "".to_string();
}
fn get_token() -> String {
    return "".to_string();
}