use crate::interaction::{
    InteractionApplicationCommandCallbackData, ApplicationCommandOption, ApplicationCommandOptionChoice, ApplicationCommandInteractionDataOption, ApplicationCommandOptionType
};
use crate::error::InteractionError;
use crate::command::Command;

use async_trait::async_trait;


pub(crate) struct Hello {}

#[async_trait(?Send)]
impl Command for Hello {
    async fn respond(&self, _options: &Option<Vec<ApplicationCommandInteractionDataOption>>, _ctx: &mut worker::RouteContext<()>) -> Result<InteractionApplicationCommandCallbackData, InteractionError>{
        Ok(InteractionApplicationCommandCallbackData {
            content: Some("Hello, world!".to_string()),
            choices: None,
            embeds: None
        })
    }

    fn name(&self) -> String{
        "hello".into()
    }

    fn description(&self) -> String {
        "Say Hello!".into()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![ApplicationCommandOption{
            name: "arg".into(), 
            autocomplete: Some(true), 
            description: "The First Argument".into(), 
            required: Some(true), 
            ty: ApplicationCommandOptionType::String,
            choices: None,
        }])
    }

    async fn autocomplete(&self, _options: &Option<Vec<ApplicationCommandInteractionDataOption>>, _ctx: &mut worker::RouteContext<()>) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        Ok(InteractionApplicationCommandCallbackData {
            content: None,
            embeds: None,
            choices: Some(vec!(ApplicationCommandOptionChoice{
                name: "option 1".into(),
                value: "test".into(),

            }))
        })

    }
}

