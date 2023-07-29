use serenity::prelude::{RwLock, TypeMap};
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

    pub async fn register_global_commands(&self, ctx: &serenity::client::Context) -> serenity::Result<Vec<serenity::model::prelude::CommandId>> {
        let res = serenity::model::application::command::Command::set_global_application_commands(ctx.http.clone(), |commands| {
            for cmd in &self.commands {
                println!("Registering {}", &cmd.name);
                commands.create_application_command(|command| {cmd.register(command)});
            }
            commands
        }).await?;

        println!("Success");

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

    pub async fn build(&mut self, client_data: Arc<RwLock<TypeMap>>) {
        if self.commands.is_none() {
            panic!("No commands were provided to the CommandHandler");
        }
        let registry = CommandRegistry {
            commands: self.commands.take().unwrap(),
        };

        let mut data_guard = client_data.write().await;
        data_guard.insert::<TypeMapCommandRegistry>(Arc::new(registry));
    }
}

pub struct TypeMapCommandRegistry;
impl serenity::prelude::TypeMapKey for TypeMapCommandRegistry {
    type Value = Arc<CommandRegistry>;
}

pub async fn extract_command_registry(ctx: &serenity::prelude::Context) ->Arc<CommandRegistry> {
    let registry = ctx.data.read().await;
    registry.get::<TypeMapCommandRegistry>().expect("No Registry found in the serenity global data TypeMap.").clone()
}