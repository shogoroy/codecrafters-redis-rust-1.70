mod resp_command;

use std::str::FromStr;

use resp_command::RespCommand;

pub struct Resp {
    pub raw: String,
    pub n_data: usize,
    pub command: RespCommand,
    pub data: Vec<String>,
}

impl Resp {
    pub fn new(raw_message: String) -> Self {
        Self::parse_message(raw_message)
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

    fn parse_message(message: String) -> Resp {
        let commands: Vec<String> = message.split("\\r\\n").map(str::to_string).collect();

        let first_message = commands.get(0).unwrap_or(&String::new());

        if first_message.as_str() == "ping" {
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

    pub fn validate_echo_command(&self) -> bool {
        match self.command {
            RespCommand::Echo => true,
            _ => false,
        }
    }
}
