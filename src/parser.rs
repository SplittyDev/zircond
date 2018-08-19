use std::str::Chars;
use crate::message::*;

pub struct IrcMessageParser;

impl IrcMessageParser {
    pub fn parse(line: String) -> IrcMessageRequest {
        let mut chars: Chars = line.chars();
        let mut tag: Option<IrcMessageTags> = None;
        let mut prefix: Option<IrcMessagePrefix> = None;
        let mut command = IrcMessageCommand::None;

        fn parse_tags(chars: &mut Chars) -> IrcMessageTags {
            let mut tags: Vec<IrcMessageTag> = Vec::new();
            let mut current_key = String::new();
            let mut current_val: Option<String> = None;
            loop {
                match chars.next() {
                    Some(chr) => {
                        match chr {
                            ' ' => {
                                let tag = IrcMessageTag(current_key, current_val.clone());
                                tags.push(tag);
                                break;
                            },
                            ';' => {
                                let tag = IrcMessageTag(current_key, current_val.clone());
                                tags.push(tag);
                                current_key = String::new();
                                current_val = None;
                            },
                            '=' => current_val = Some(String::new()),
                            _ => {
                                match current_val {
                                    Some(ref mut buf) => buf.push(chr),
                                    None => current_key.push(chr),
                                };
                            }
                        }
                    },
                    None => panic!("Unexpected end of tag."),
                };
            }
            IrcMessageTags::Many(tags)
        }

        fn parse_prefix(chars: &mut Chars) -> IrcMessagePrefix {
            let mut buf = String::new();
            loop {
                match chars.next() {
                    Some(chr) => {
                        match chr {
                            ' ' => break,
                            _ => buf.push(chr),
                        }
                    },
                    None => panic!("Unexpected end of prefix."),
                };
            }
            IrcMessagePrefix(buf)
        }

        fn parse_command_name(chars: &mut Chars, current_char: char) -> String {
            let mut command_name = String::new();
            command_name.push(current_char);
            loop {
                match chars.next() {
                    Some(chr) => {
                        match chr {
                            ' ' => break,
                            _ => command_name.push(chr),
                        }
                    },
                    None => panic!("Unexpected end of command.")
                }
            }
            command_name
        }

        fn parse_command_parameter(chars: &mut Chars) -> Option<String> {
            let mut parameter = String::new();
            let mut trailing_parameter = false;
            loop {
                match chars.next() {
                    Some(chr) => {
                        match chr {
                            ' ' if !trailing_parameter => break,
                            ':' => trailing_parameter = true,
                            _ => parameter.push(chr),
                        }
                    },
                    None => break,
                }
            }
            if parameter.len() > 0 {
                Some(parameter.trim().to_owned())
            } else {
                None
            }
        }

        fn parse_command_parameters(mut chars: &mut Chars) -> Vec<String> {
            let mut parameters: Vec<String> = Vec::new();
            while let Some(parameter) = parse_command_parameter(&mut chars) {
                parameters.push(parameter);
            }
            parameters
        }

        fn parse_command(mut chars: &mut Chars, current_char: char) -> IrcMessageCommand {
            let command_name = parse_command_name(&mut chars, current_char);
            let parameters = parse_command_parameters(&mut chars);

            macro_rules! extract {
                ($params:expr; $command:ident $pos:expr => REQ $name:expr) => (
                    $params
                        .iter()
                        .nth($pos)
                        .expect(&format!("Command '{}' MUST include the {}!", stringify!($command), $name))
                        .to_owned()
                );
                ($params:expr; $command:ident $pos:expr => OPT $name:expr) => (
                    $params
                        .iter()
                        .nth($pos)
                        .map_or(None, |param| Some(param.to_owned()))
                );
            }

            macro_rules! validate {
                ($params:expr; $command:ident $pos:expr => MUST EQ $expected:expr; $message:expr) => {{
                    let tmp = $params
                        .iter()
                        .nth($pos)
                        .expect(&format!("Failed validation for command '{}'", stringify!($command)))
                        == $expected;
                    if !tmp {
                        panic!("Failed validation assertion for command '{}': {}", stringify!($command), $message)
                    }
                }};
                ($params:expr; $command:ident $pos:expr => SHOULD EQ $expected:expr; $message:expr) => {{
                    let tmp = $params
                        .iter()
                        .nth($pos)
                        .expect(&format!("Failed validation for command '{}'", stringify!($command)))
                        == $expected;
                    if !tmp {
                        // println!("Failed optional validation for command '{}': {}", stringify!($command), $message)
                    }
                }};
            }

            match command_name.as_ref() {
                "NICK" => {
                    let nickname = extract!(parameters; NICK 0 => REQ "nickname");
                    IrcMessageCommand::Nick(nickname)
                },
                "USER" => {
                    let username = extract!(parameters; USER 0 => REQ "username");
                    validate!(parameters; USER 1 => SHOULD EQ "0"; "Second parameter should equal '0'");
                    validate!(parameters; USER 2 => SHOULD EQ "*"; "Third parameter should equal '*'");
                    let realname = extract!(parameters; USER 3 => OPT "realname");
                    IrcMessageCommand::User(username, realname)
                }
                "JOIN" => {
                    let channel = extract!(parameters; JOIN 0 => REQ "channel name");
                    IrcMessageCommand::Join(channel)
                }
                "PING" => {
                    let challenge = extract!(parameters; JOIN 0 => REQ "challenge");
                    IrcMessageCommand::Ping(challenge)   
                }
                _ => {
                    println!("Unimplemented: {}", command_name);
                    IrcMessageCommand::None
                }
            }
        }
        
        loop {
            let chr = chars.next();
            if chr.is_none() {
                break;
            }

            match chr.unwrap() {

                // Tag
                '@' => tag = Some(parse_tags(&mut chars)),

                // Prefix
                ':' => prefix = Some(parse_prefix(&mut chars)),

                // Message
                chr => {
                    command = parse_command(&mut chars, chr);
                    break;
                }
            };
        }

        IrcMessageRequest::new(command, prefix, tag)
    }
}