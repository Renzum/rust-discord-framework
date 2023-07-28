use std::future::Future;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandType as SerenityCommandType;

pub enum CommandType {
    SlashCommand,
    MessageCommand,
    UserCommand,
}

impl CommandType {
    pub fn is_slash(&self) -> bool {
        match self {
            Self::SlashCommand => true,
            _ => false,
        }
    }
}

pub struct Command {
    pub name: String,
    pub kind: CommandType,
    pub description: std::option::Option<String>,
    pub inner: fn(super::ApplicationContext) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send>>,
    //pub options: Vec<> TODO
}

impl Command {
    pub fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        let command = command.name(&self.name);
        match self.kind {
            CommandType::MessageCommand => command.kind(SerenityCommandType::Message),
            CommandType::SlashCommand => command.kind(SerenityCommandType::ChatInput),
            CommandType::UserCommand => command.kind(SerenityCommandType::User),
        };

        if self.description.is_some() {
            match self.kind {
                CommandType::SlashCommand => command.description(self.description.as_ref().unwrap()),
                _ => panic!("A Message or User Command cannot have a description."),
            };
        }

        command
    }
}