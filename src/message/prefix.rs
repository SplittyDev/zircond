pub struct IrcMessagePrefix(pub String);

impl ToString for IrcMessagePrefix {
    fn to_string(&self) -> String {
        let mut buf = String::with_capacity(self.0.len() + 1);
        buf.push(':');
        buf.push_str(self.0.as_ref());
        buf
    }
}