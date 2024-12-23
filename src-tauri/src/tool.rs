#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use rfd::FileDialog;
use std::process::Command;
use std::fs;

use std::{future::Future, thread};
use std::collections::HashMap;
use std::net::{TcpStream,SocketAddr};
use std::io::Error;
use std::time::Duration;
use std::fmt;
// use std::net::ipAddr;
use reqwest::{Client, StatusCode};
use std::sync::{Mutex, Arc};
use calamine::{open_workbook_auto,Data,Range,Reader,Error as CalamineError};

use std::path::{Path, PathBuf};

#[tauri::command]
pub fn service(name: &str) -> String {
    let result = format!("主任, {}! 开始为您服务!", name);
    // ok(result.to_string());
    println!("{}", result);
    result
}
#[tauri::command]
pub fn select_local_file() -> String {  
    let path = "error path";
    let file = FileDialog::new()
        .add_filter("image", &["png", "jpg", "jpeg"])
        .add_filter("dmg", &["dmg"])
        .add_filter("hap", &["hap"])
        .set_directory("/")
        .pick_file();
    let local_path = match file {
        Some(file) => format!("{}", file.display()),
        None => "No file selected".to_string(),
    };
    // if let Some(file) = file {
    //     println!("Selected file: {:?}", file.display());
    //     let local_path = format!("{}", file.display());

    //     path = local_path.as_str();
    // }
    println!("Selected file: {}", local_path);
    local_path
}
#[tauri::command]
pub fn select_local_ip_list() -> String {  
    let path = "error path";
    let file = FileDialog::new()
        .add_filter("text", &["txt","rtf","xlsx"])
        .set_directory("/")
        .pick_file();
    let local_path = match file {
        Some(file) => format!("{}", file.display()),
        None => "No file selected".to_string(),
    };
    if local_path.ends_with(".txt") {
        let contents  = fs::read_to_string(local_path);
        let result = match contents {
            Ok(content) => content,
            Err(e) => "".to_string(),
        };
        // println!("result = {}",result);
        return result;
    } else if local_path.ends_with(".xlsx") {
        let sce = PathBuf::from(local_path);
        let mut xl = open_workbook_auto(&sce).unwrap();
        let range = xl.worksheet_range("Sheet1").unwrap();
        let mut column_data = Vec::new();
        let header = range.headers().unwrap();
        for (row_num,row) in range.rows().enumerate() {
            for (col_num,cell) in row.iter().enumerate() {
                match *cell {
                    Data::String(ref s) => column_data.push(s.as_str().into()),
                    Data::Float(f) => column_data.push(f.to_string()),
                    Data::Int(i) => column_data.push(i.to_string()),
                    _ => println!("unknown data type"),
                }
            }
        }
        // column_data.iter()
        // .map(|num| num.to_string())
        // .collect::<Vec<String>>()
        // .join("\n")
        println!("column_data = {:#?}", column_data);
        return column_data.join("\n");
    }

    String::from("error")

    
    // local_path
}

#[tauri::command]
pub fn install_harmony_package(path: &str) -> bool  {
    println!("install_harmony_package action {}",path);

    let output = Command::new("hdc")
    //    .arg("185.199.108.133")
       .arg("app")
       .arg("install")
       .arg(path)
       .output()
       .expect("failed to execute ping process");
    if output.status.success() {
        println!("安装成功");
        // 成功，解析输出
        // Ok(String::from_utf8_lossy(&output.stdout).to_string())
        return true;
    } else {
        // 失败，解析错误输出
        println!("安装失败");
        // Err(String::from_utf8_lossy(&output.stderr).to_string())
        return false;
    }

    // let output  = Command::new("hdc app install")
    //     .arg(path)
    //     .output()
    //     .expect("error");
    // // println!("local_path = {}",path);
    // // cmd.arg("app");
    // // cmd.arg("install");
    // // cmd.arg(path);

    // // let output = cmd.output().expect("error");
    //                 // .expect("failed to install harmony package");
    // if output.status.success() {
    //     println!("安装成功");
    // } else {
    //     println!("安装失败");
    // }
}

#[tauri::command]
pub fn toast(content: &str) {
    // Toast::new(content).show();
}



#[tauri::command]
pub fn is_ip_reachable(ip: &str) -> bool{
    // println!("is_ip_reachable  is_ip_reachable: {}", ip);
    // true
    let ip_port = format!("{}:{}", ip, 80);
    let addr = ip_port.parse().unwrap();
    match TcpStream::connect_timeout(&addr,Duration::from_secs(2)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
#[tauri::command]
pub fn ip_list_reachable(list: Vec<&str>) -> HashMap<String, bool> {
    // println!("ip_list: {:#?}", list);
    let mut return_map: HashMap<String, bool> = HashMap::new();
    let result_map: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(return_map));
    let mut handles = vec![];
    let ip_list_clone = list.clone();
    for (index,ip) in list.iter().enumerate() {
        let current_map = Arc::clone(&result_map);
        let current_ip = ip.to_string();
        let handle = thread::spawn( move ||  {
            let mut map = current_map.lock().unwrap(); 
           let is_avaliable: bool = is_ip_reachable(&current_ip);
        //    let key = ip.to_string(); 
           map.insert((&current_ip).to_string(), is_avaliable);
        });
        handles.push(handle);
    }
    for handle in handles { 
        handle.join().unwrap();
    }
    let result = result_map.lock().unwrap();
    return result.clone();
    // returun_map
    // result_map
}


