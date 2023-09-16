use super::model::Resp;

pub fn validate_echo_command(rr: &Resp) -> bool {
    rr.command == "ECHO"
}
