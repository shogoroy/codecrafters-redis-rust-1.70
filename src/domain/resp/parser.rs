use std::str::FromStr;

use super::model::Resp;
use crate::domain::resp_command::RespCommand;

pub fn parse_message(message: String) -> Resp {
    let commands: Vec<String> = message.split("\\r\\n").map(str::to_string).collect();

    let first_message = commands.get(0).unwrap_or(&String::new());

    if first_message == "ping" {
        let data = commands.get(1..).unwrap_or_default().to_vec();
        return Resp {
            raw: message,
            n_data: 0,
            command: RespCommand::PING,
            data,
        };
    }

    let n_data: usize = first_message.replace("*", "").parse().unwrap_or(0);

    let mut contents: Vec<String> = vec![String::new(); n_data];
    for i in 0..n_data {
        contents[i] = commands.get((i + 1) * 2).unwrap().to_string();
    }
    let command = contents.get(0).unwrap_or(&String::new()).to_string();
    let data = contents.get(1..).unwrap_or_default().to_vec();

    let rr = Resp {
        raw: message,
        n_data,
        command: RespCommand::from_str(command.as_str()).unwrap(),
        data,
    };

    rr
}
