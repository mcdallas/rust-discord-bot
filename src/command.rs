use serde::{Deserialize, Serialize};
use crate::commands;
use crate::interaction::*;
use crate::error::InteractionError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub(crate) trait Command {
    async fn respond(&self, _options: &Option<Vec<ApplicationCommandInteractionDataOption>>, _ctx: &mut worker::RouteContext<()>) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        // Implement the command logic here
        unimplemented!()
    }

    fn name(&self) -> String {
        // The command name, ie `return "greet".to_string()` for /greet
        unimplemented!()
    }

    fn description(&self) -> String {
        // A short description
        unimplemented!()
    }
    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        // add any arguments/choices here, more info at https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
        unimplemented!()
    }

    async fn autocomplete(&self, _options: &Option<Vec<ApplicationCommandInteractionDataOption>>, _ctx: &mut worker::RouteContext<()>) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        // If your command supports autocomplete implement the logic here
        unimplemented!()
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RegisteredCommand {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) options: Option<Vec<ApplicationCommandOption>>
}


pub(crate) fn init_commands() -> Vec<Box<dyn Command + Sync>> {
    let mut v : Vec<Box<dyn Command + Sync>> = Vec::new();
    v.push(Box::new(commands::hello::Hello {}));
    v
}