use crate::message::IrcMessageTag;

pub enum IrcMessageTags {
    One(IrcMessageTag),
    Many(Vec<IrcMessageTag>),
}

impl ToString for IrcMessageTags {
    fn to_string(&self) -> String {

        // Map multiple tags to their string representations
        let tags = match self {
            IrcMessageTags::Many(tags) => Some(tags.iter().map(|tag| tag.to_string()).collect::<Vec<_>>()),
            _ => None,
        };

        // Get the exact combined length of all tags
        let len = match self {
            IrcMessageTags::One(tag) => tag.to_string().len(),
            IrcMessageTags::Many(_) => tags.iter().map(|tag| tag.len()).fold(0, |a, b| a + b),
        };

        // Get the tag separator count
        let len_separators = match self {
            IrcMessageTags::Many(tags) => if tags.len() > 1 { tags.len() - 1 } else { 0 },
            _ => 0,
        };

        // Create a perfectly fitted string
        let mut buf = String::with_capacity(len + len_separators + 1);

        // Push the tag prefix '@'
        buf.push('@');

        match self {

            // Push a single tag to the string buffer
            IrcMessageTags::One(tag) => buf.push_str(tag.to_string().as_ref()),

            // Push multiple tags to the string buffer
            IrcMessageTags::Many(_) => {

                // Unwrap the tag strings.
                // This is guaranteed to work since the tags variable is
                // always Some(_) if the IrcMessageTags variant is Many(_)
                let tags = tags.unwrap();

                for (i, tag) in tags.iter().enumerate() {

                    // Push separator if necessary
                    if i > 0 {
                        buf.push(';');
                    }

                    buf.push_str(tag);
                }
            }
        }

        buf
    }
}