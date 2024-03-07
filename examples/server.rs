use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::Type::{INFO, LOCAL, NotKnow, SYNC};

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:10086";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");
    loop {
        let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            // 读取来自客户端的数据
            let bytes_read = socket.read(&mut buffer).await.expect("Failed to read data from socket");
            if bytes_read != 0 {
                // 处理接收到的数据
                let received_data = &buffer[..bytes_read];

                let result = String::from_utf8_lossy(received_data).as_ref().replace('$', "");
                println!("result: {:?}", result);
                let common: CommonStruct = result.try_into().unwrap();
                println!("common:{:?}", common);

                socket.write("S168#865016401138730#8a36#0004#JUST$".as_bytes()).await.expect("Failed to write response to socket");

            }
        });
    }

}


#[derive(Debug,Clone)]
pub struct CommonStruct {
    //设备id
    pub device_id: String,
    //设备imei
    pub imei: String,
    //流水号
    pub serial_number: String,
    //消息长度
    pub length: String,
    //消息类型
    pub information: Type,
    //消息类型值
    pub type_info: String,
    //消息内容
    pub content: String
}

impl TryFrom<String> for CommonStruct {
    type Error = ();

    fn try_from(result: String) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = result.split('#').collect();
        //解析数据
        if fields.len() >= 5 {
            let device_id = fields[0].to_string();
            let imei = fields[1].to_string();
            let serial_number = fields[2].to_string();
            let length = fields[3].to_string();
            let other = fields[4].to_string();

            if !other.is_empty() {
                //获取类型 other: SYNC:0004;STATUS:13,100
                let infos:Vec<&str> = other.split(';').collect();
                if infos.len() >= 2 {
                    let info = infos[0]; //"SYNC:08c0"
                    let content = infos[1].to_string(); // STATUS:13,100
                    if !info.is_empty() {
                        let i:Vec<&str> = info.split(":").collect();

                        return Ok(CommonStruct {
                            device_id,
                            imei,
                            serial_number,
                            length,
                            information: Type::from(i[0]),
                            type_info: i[1].to_string(),
                            content,
                        })
                    }
                }
            }
        };
        Err(())
    }
}

#[derive(Debug,Clone)]
pub enum Type {
    SYNC,
    LOCAL,
    INFO,
    NotKnow
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "SYNC" => SYNC,
            "LOCA" => LOCAL,
            "INFO,ITEM" => INFO,
            _ => NotKnow
        }
    }
}