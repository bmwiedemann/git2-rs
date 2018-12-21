use std::process::{Command, Stdio};
pub struct Cred {
}
pub struct CredentialHelper {
}
impl CredentialHelper {
    fn execute_cmd(&self, cmd: &str, username: &Option<String>)
                   -> (Option<String>, Option<String>) {
        macro_rules! my_try( ($e:expr) => (
            match $e {
                Ok(e) => e,
                Err(e) => {
                    return (None, None)
                }
            }
        ) );
        let mut c = Command::new("sh");
        let mut p = match c.spawn() {
            Ok(p) => p,
            Err(e) => {
                match c.spawn() {
                    Ok(p) => p,
                    Err(e) => {
                        return (None, None);
                    }
                }
            }
        };
        let output = my_try!(p.wait_with_output());
        self.parse_output(output.stdout)
    }
    fn parse_output(&self, output: Vec<u8>) -> (Option<String>, Option<String>) {
        let mut username = None;
        let mut password = None;
        for line in output.split(|t| *t == b'\n') {
            let mut parts = line.splitn(2, |t| *t == b'=');
            let value = match parts.next() {
                Some(s) => s,
                None => {
                    continue
                }
            };
        }
        (username, password)
    }
}
