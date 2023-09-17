use std::str::FromStr;

pub enum RespCommand {
    ECHO,
    PING,
}

impl FromStr for RespCommand {
    type Err = ();

    fn from_str(command: &str) -> Result<RespCommand, Self::Err> {
        match command {
            "ECHO" => Ok(RespCommand::ECHO),
            "PING" => Ok(RespCommand::PING),
            _ => Err(()),
        }
    }
}
