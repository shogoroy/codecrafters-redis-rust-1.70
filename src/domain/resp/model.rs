pub struct Resp {
    pub raw: String,
    pub n_data: usize,
    pub command: crate::domain::resp_command::RespCommand,
    pub data: Vec<String>,
}

impl Resp {
    pub fn new(raw_message: String) -> Self {
        super::parser::parse_message(raw_message)
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
}
