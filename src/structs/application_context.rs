use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
pub struct ApplicationContext {
    pub name: String,
    pub ctx: serenity::client::Context,
    //pub options: Vec<CommandDataOption>, TODO
    pub interaction: ApplicationCommandInteraction
}

impl ApplicationContext {
    pub fn new(ctx: serenity::prelude::Context, interaction: ApplicationCommandInteraction) -> Self {
        ApplicationContext {
            name: interaction.data.name.clone(),
            ctx: ctx,
            interaction: interaction,
        }
    }
}