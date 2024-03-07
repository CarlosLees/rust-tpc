// 作为客户端 接收设备消息 回复消息

use crate::modal::CommonStruct;

#[derive(Debug)]
pub struct Heartbeat {
    pub common: CommonStruct,
    //状态
    pub status: String,
    //电量
    pub electricity: String,
    //信号
    pub signal: String,
}

impl TryFrom<CommonStruct> for Heartbeat {
    type Error = ();

    fn try_from(value: CommonStruct) -> Result<Self, Self::Error> {
        if !value.content.is_empty() {
            let content = value.clone().content;
            let content_vec:Vec<&str> = content.split(":").collect();
            if content_vec.len() >= 2 {
                let status = content_vec[0].to_string();
                let status_info:Vec<&str> = content_vec[1].split(",").collect();
                let electricity = status_info[0].to_string();
                let signal = status_info[1].to_string();
                let heart = Heartbeat {
                    common: value,
                    status,
                    electricity,
                    signal
                };
                return Ok(heart);
            }
        }
        Err(())
    }
}