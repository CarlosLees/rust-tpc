use crate::modal::Type::{INFO, LOCAL, NotKnow, SYNC};

pub mod server;
pub mod client;

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