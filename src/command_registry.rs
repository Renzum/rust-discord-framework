use serenity::prelude::RwLock;
use std::sync::Arc;

use super::structs::Command;

pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn builder() -> CommandRegistryBuilder {
        CommandRegistryBuilder { commands: None }
    }

    pub fn locate_command(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|cmd| {
            cmd.name == name
        })
    }

    pub async fn register_global_commands(&self, ctx: serenity::client::Context) -> serenity::Result<Vec<serenity::model::prelude::CommandId>> {
        let res = serenity::model::application::command::Command::set_global_application_commands(ctx.http.clone(), |commands| {
            for cmd in &self.commands {
                commands.create_application_command(|command| {cmd.register(command)});
            }
            commands
        }).await?;

        Ok(res.iter().map(|cmd| {
            cmd.id
        }).collect())
    }

}

#[derive(Default)]
pub struct CommandRegistryBuilder {
    commands: Option<Vec<Command>>,
}

impl CommandRegistryBuilder {

    pub fn add_command(&mut self, cmd: Command) -> &mut Self {
        let mut commands = self.commands.as_mut();
        if commands.is_none() {
            self.commands = Some(Vec::new());
            commands = self.commands.as_mut();
        }

        commands.unwrap().push(cmd);
        self
    }

    pub fn build(&mut self) -> CommandRegistry {
        if self.commands.is_none() {
            panic!("No commands were provided to the CommandHandler");
        } else {
            CommandRegistry {
                commands: self.commands.take().unwrap(),
            }
        }
    }
}

pub struct TypeMapCommandRegistry;
impl serenity::prelude::TypeMapKey for TypeMapCommandRegistry {
    type Value = Arc<RwLock<CommandRegistry>>;
}

pub async fn extract_command_registry(ctx: &serenity::prelude::Context) ->Arc<RwLock<CommandRegistry>> {
    let registry = ctx.data.read().await;
    registry.get::<TypeMapCommandRegistry>().expect("No Registry found in the serenity global data TypeMap.").clone()
}