use std::process::Command;

#[allow(dead_code)]
#[tauri::command]
pub fn ping_ip(ip:&str) -> bool {
// fn ping(ip:&str) -> Result<String,String> {
    let output = Command::new("ping")
       .arg(ip)
       .arg("-c")
       .arg("4")
       .output()
       .expect("failed to execute ping process");
    if output.status.success() {
        println!("ping {} result: {}", ip, true);
        // 成功，解析输出
        // Ok(String::from_utf8_lossy(&output.stdout).to_string())
        return true;
    } else {
        // 失败，解析错误输出
        println!("ping {} result: {}", ip, false);
        // Err(String::from_utf8_lossy(&output.stderr).to_string())
        return false;
    }
}

#[allow(dead_code)]
#[tauri::command]
pub fn ping_ip_list(ip_list: Vec<&str>) -> Vec<bool> {
    let mut result_list = Vec::new();
    for ip in ip_list {
        let result = ping_ip(ip);
        // println!("ping {} result: {}", ip, result);
        result_list.push(result);
    }
    return result_list;
}
#[allow(dead_code)]
#[tauri::command]
pub fn test_terminal()  -> bool {
    println!("这里调用了终端");
    // let output  = Command::new("hdc")
    // .arg("-v")
    // .output()
    // .expect("Failed to execute hdc process");
    // if output.status.success() {
    //     println!("安装成功");
    // } else {
    //     println!("安装失败");
    // }
    let ip = "185.199.108.133";
    let output = Command::new("hdc")
    //    .arg("185.199.108.133")
       .arg("app")
       .arg("install")
       .arg("")
       .output()
       .expect("failed to execute ping process");
    if output.status.success() {
        println!("ping {} result: {}", "185.199.108.133", true);
        // 成功，解析输出
        // Ok(String::from_utf8_lossy(&output.stdout).to_string())
        return true;
    } else {
        // 失败，解析错误输出
        println!("ping {} result: {}", "185.199.108.133", false);
        // Err(String::from_utf8_lossy(&output.stderr).to_string())
        return false;
    }
}