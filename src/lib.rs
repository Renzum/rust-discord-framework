use tokio::task::JoinHandle;
use serenity::model::application::interaction::Interaction;

pub mod structs;
pub use structs::*;

mod command_registry;
pub use command_registry::CommandRegistry;

pub mod macros {
    pub use command_framework_macros::*;
}


pub async fn handle_command(ctx: serenity::prelude::Context, application_interaction: Interaction) -> Option<JoinHandle<()>> {
    let interaction = if let Interaction::ApplicationCommand(inter) = application_interaction {
        inter
    } else {
        return None;
    };
    let registry = command_registry::extract_command_registry(&ctx).await;

    let app_context = ApplicationContext::new(ctx, interaction);
    let registry = registry.clone();

    let command = registry.locate_command(&app_context.name);


    match command {
        Some(command) => Some(tokio::spawn((command.inner)(app_context))),
        None => None
    }
}

pub async fn register_commands(ctx: &serenity::prelude::Context) {
    let registry = command_registry::extract_command_registry(ctx).await;

    let _commands = registry.register_global_commands(ctx).await;
    if let Err(why) = _commands {
        eprintln!("{:#?}", why);
    }
}