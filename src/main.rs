use std::sync::Arc;
use axum::{Router, Server};
use axum::extract::Path;
use axum::routing::get;
use chrono::Local;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;
use crate::modal::{CommonStruct, Type};
use crate::modal::client::Heartbeat;

mod modal;

static SOCKET_MAP: Lazy<Arc<DashMap<String, Arc<Mutex<OwnedWriteHalf>>>>> = Lazy::new(|| {
    Arc::new(DashMap::new())
});

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:10086";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");

    let axum_server = tokio::spawn(run_axum_server());
    let tcp_listener = tokio::spawn(handle_tcp_connections(listener));

    tokio::select! {
        _ = axum_server => { /* axum server completed */ }
        _ = tcp_listener => { /* TCP listener completed */ }
    }
}

async fn run_axum_server() {
    let app = Router::new().route("/:imei", get(device_location));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn device_location(Path(imei): Path<String>) -> String {
    if let Some(res) = SOCKET_MAP.get::<String>(&imei) {
        let i = res.lock().await.write("S168#865016401138730#8a36#0004#JUST$".as_bytes()).await.unwrap();
        println!("{}", i);
    }

    format!("Hello, world! {}",imei)
}

async fn handle_tcp_connections(listener: TcpListener) {
    loop {
        let (socket, _) = listener.accept().await.expect("Failed to accept connection");

        handle_client(socket).await;
    }
}

async fn handle_client(socket: TcpStream) {
    let (mut read, write) = socket.into_split();
    let write_mutex = Arc::new(Mutex::new(write));

    let socket_map = SOCKET_MAP.clone();

    let join_handle= tokio::spawn(async move {
        let mut buffer = [0; 1024];
        // 读取来自客户端的数据
        let bytes_read = read.read(&mut buffer).await.expect("Failed to read data from socket");
        if bytes_read != 0 {
            // 处理接收到的数据
            let received_data = &buffer[..bytes_read];

            let result = String::from_utf8_lossy(received_data).as_ref().replace('$', "");
            println!("result: {:?}", result);

            let common: CommonStruct = result.try_into().unwrap();

            return Some((handle_information(common.clone()), common.clone().imei));
        }
        return None;
    });

    if let Some(result) = join_handle.await.unwrap() {
        println!("imei: {:?}", result.1);
        // 判断消息类型 返回对应的格式 发送响应给客户端
        socket_map.insert(result.1.clone(),write_mutex.clone());

        let mut write_guard = write_mutex.lock().await;
        write_guard.write(result.0.as_bytes()).await.expect("Failed to write response to socket");
    }
}

//处理消息类型
fn handle_information(common: CommonStruct) -> String {
    let current_time = Local::now().format("%Y%m%d%H%M%S").to_string();

    println!("{:?}",current_time);
    return match common.information {
        Type::SYNC => {
            // 心跳信息
            let heartbeat: Heartbeat = common.try_into().unwrap();
            println!("{:?}", heartbeat);
            //回复消息
            let common = heartbeat.common;
            let response_info = format!("ACK^SYNC,{}$", current_time);
            println!("response_info:{:?}", response_info);

            // S168#000000000000000#002a#0017#ACK^SYNC,20231110005435$
            let response = format!("{}#{}#{}#{:04X}#{}", common.device_id, common.imei, common.serial_number, response_info.len() - 1, response_info);
            println!("{:?}", response);
            response
        },
        Type::LOCAL => {
            println!("{:?}",common);
            String::new()
        },
        Type::INFO => {
            println!("{:?}",common);
            String::new()
        },
        _ => {
            String::new()
        }
    }
}