use super::model::Resp;

pub fn parse_message(message: String) -> Resp {
    let commands: Vec<String> = message.split("\\r\\n").map(str::to_string).collect();

    let n_data: usize = commands
        .get(0)
        .unwrap_or(&String::new())
        .replace("*", "")
        .parse()
        .unwrap_or(0);

    let mut contents: Vec<String> = vec![String::new(); n_data];
    for i in 0..n_data {
        contents[i] = commands.get((i + 1) * 2).unwrap().to_string();
    }
    let command = contents.get(0).unwrap_or(&String::new()).to_string();
    let data = contents.get(1..).unwrap_or_default().to_vec();

    let rr = Resp {
        raw: message,
        n_data,
        command,
        data,
    };

    rr
}
