#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use local_ip_address::local_ip;
use std::net::{TcpListener};
use std::fs;
use std::io::{Write};
use std::io;
use std::{thread, time::Duration};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::sync::Mutex;
use std::sync::MutexGuard;
use tauri::{Window};

static mut MTX: Mutex<bool> = Mutex::new(false);

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]   
fn get_current_ip_address() -> String {
    local_ip().unwrap().to_string()
}

#[tauri::command]   
fn terminate_server(){
    unsafe {
        let mut mutex: MutexGuard<bool> = MTX.lock().unwrap();
        *mutex = true; 
    }
}

#[tauri::command]
fn print_selected_files(file_path_string_array: Vec<String>){
    for file_path in file_path_string_array {
        read_file(&file_path);
        //println!("Files received: {}", file_path);
    }
}

#[tauri::command]
fn create_settings_directory(path: String) -> String{
    match fs::create_dir(path){
        Ok(_) => return "Success".to_string(),
        Err(err) => return format!("{}",err)
    }
}

#[tauri::command]
fn create_secret() -> String{
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(20)
    .map(char::from)
    .collect()
}

#[tauri::command]
fn check_for_config(path: String) -> String{
    match fs::read_dir(&path){
        Ok(_) => {
            let config_path: String = format!("{}\\config.frgmshr",path);
            match fs::read_to_string(config_path){
                Ok(res) => return format!("{}",res),
                Err(err) => return format!("{}",err)
            }
        },
        Err(err) => return format!("{}",err)
    }
}

#[tauri::command]
async fn create_server(window: Window) {
    unsafe {
        let mut mutex: MutexGuard<bool> = MTX.lock().unwrap();
        *mutex = false;
    }
    let mut timeout: i32 = 15;
    let listener = TcpListener::bind("127.0.0.1:7877").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    for stream in listener.incoming() {
      match stream {
        Ok(mut s) => {
            println!("incoming connection!");
            s.write(b"Hello Client!").unwrap();
            break;
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            unsafe {
                let mut mutex: MutexGuard<bool> = MTX.lock().unwrap();
                if *mutex == true { 
                    *mutex = false;
                    println!("Server terminated preemptively!");
                    break; 
                };
            }
            if timeout <= 0 { 
                window.emit("frgshr://connection-validity-automatic-closure", Payload { message: "connection automatically closed".to_string() }).unwrap();
                break;
            };
            println!("Thread sleeping and waiting for Client to join for {} seconds ...", timeout.to_string());
            window.emit("frgshr://connection-validity", Payload { message: timeout.to_string() }).unwrap();
            thread::sleep(Duration::from_millis(1000));
            timeout -= 1;
            continue;
        }
        Err(e) => panic!("encountered IO error: {}", e),
    }
  }
  println!("Server closed.");
}

fn read_file(file_path_string: &String){
    let data = fs::read(file_path_string).expect("Unable to read file");
    println!("{}", data.len());
}


// fn handle_connection(stream: TcpStream) {
//    println!("Yeah it connected!")
// }

fn main() /*-> std::io::Result<()>*/ {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_current_ip_address, 
            print_selected_files, 
            create_server,
            create_settings_directory,
            check_for_config,
            create_secret,
            terminate_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
