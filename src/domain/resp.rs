mod resp_command;
mod resp_data_type;

use std::str::FromStr;

use resp_command::RespCommand;
use resp_data_type::RespDataType;

pub struct Resp {
    pub raw: String,
    pub n_data: usize,
    pub command: RespCommand,
    pub data: Vec<String>,
}

impl Resp {
    pub fn new(raw_message: String) -> Self {
        Self::parse_message(raw_message).unwrap()
    }

    pub fn echo(&self) -> String {
        let message = self.data.join(" ");

        if (self.data.len() + 1) != self.n_data {
            self.error("ERR wrong number of arguments for command");
            println!(
                "ERR wrong number of arguments for command: {}, {}",
                self.data.len(),
                self.n_data
            );
        }

        message
    }

    pub fn error(&self, message: &str) -> String {
        String::from("-Error ") + message
    }

    fn parse_message(message: String) -> Option<Resp> {
        let commands: Vec<String> = message.split("\\r\\n").map(str::to_string).collect();

        let first_message = commands.get(0).unwrap();
        match Self::get_resp_data_type(&message) {
            Ok(v) => match v {
                RespDataType::SimpleStrings => {
                    // TODO: if first_message.as_str() == "ping" {
                    let data = commands.get(1..).unwrap_or_default().to_vec();
                    let resp = Resp {
                        raw: message,
                        n_data: 0,
                        command: RespCommand::Ping,
                        data,
                    };
                    Some(resp)
                }
                RespDataType::Arrays => {
                    let n_data: usize = first_message.replace("*", "").parse().unwrap_or(0);

                    let mut contents: Vec<String> = vec![String::new(); n_data];
                    for i in 0..n_data {
                        contents[i] = commands.get((i + 1) * 2).unwrap().to_string();
                    }
                    let command = contents.get(0).unwrap_or(&String::new()).to_string();
                    let data = contents.get(1..).unwrap_or_default().to_vec();

                    let resp = Resp {
                        raw: message,
                        n_data,
                        command: RespCommand::from_str(command.as_str()).unwrap(),
                        data,
                    };
                    Some(resp)
                }
                _ => None, // FIXME
            },
            Err(_) => None,
        }
    }

    fn get_resp_data_type(raw: &String) -> Result<RespDataType, &str> {
        match raw.chars().next().unwrap() {
            '+' => Ok(RespDataType::SimpleStrings),
            '-' => Ok(RespDataType::SimpleErrors),
            ':' => Ok(RespDataType::Integers),
            '$' => Ok(RespDataType::BulgStrings),
            '*' => Ok(RespDataType::Arrays),
            '_' => Ok(RespDataType::Nulls),
            '#' => Ok(RespDataType::Booleans),
            ',' => Ok(RespDataType::Doubles),
            '(' => Ok(RespDataType::BigNumbers),
            '!' => Ok(RespDataType::BulkErrors),
            '=' => Ok(RespDataType::VerbatimStrings),
            '%' => Ok(RespDataType::Maps),
            '~' => Ok(RespDataType::Sets),
            '>' => Ok(RespDataType::Pushes),
            _ => Err(""),
        }
    }

    pub fn response(self) -> String {
        match self.command {
            RespCommand::Echo => self.echo(),
            RespCommand::Ping => String::from("+PONG"),
        }
    }
}
