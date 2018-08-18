pub struct IrcMessageTag(pub String, pub Option<String>);

impl ToString for IrcMessageTag {
    fn to_string(&self) -> String {

        // Pre-calculate the exact tag size
        let len = self.0.len() + self.1.as_ref().map_or(0, |tag| tag.len() + 1);

        // Allocate a perfectly fitted string
        let mut buf = String::with_capacity(len);

        // Push the tag parts
        buf.push_str(self.0.as_ref());
        if self.1.is_some() {
            buf.push('=');
            buf.push_str(self.1.as_ref().unwrap().as_ref());
        }
        
        buf
    }
}