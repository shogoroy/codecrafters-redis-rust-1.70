use std::str::FromStr;

pub enum RespCommand {
    Echo,
    Ping,
}

impl FromStr for RespCommand {
    type Err = ();

    fn from_str(command: &str) -> Result<RespCommand, Self::Err> {
        match command {
            "ECHO" => Ok(RespCommand::Echo),
            "PING" => Ok(RespCommand::Ping),
            _ => Err(()),
        }
    }
}
