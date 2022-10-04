use crate::interaction::{
    InteractionApplicationCommandCallbackData, ApplicationCommandOption, ApplicationCommandOptionChoice, ApplicationCommandOptionType
};
use crate::error::InteractionError;
use crate::command::{Command, CommandInput};

use async_trait::async_trait;


pub(crate) struct Hello {}

#[async_trait(?Send)]
impl Command for Hello {
    async fn respond(&self, _input: &CommandInput) -> Result<InteractionApplicationCommandCallbackData, InteractionError>{
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

    async fn autocomplete(&self,  _input: &CommandInput) -> Result<Option<InteractionApplicationCommandCallbackData>, InteractionError> {
        Ok(Some(InteractionApplicationCommandCallbackData {
            content: None,
            embeds: None,
            choices: Some(vec!(ApplicationCommandOptionChoice{
                name: "option 1".into(),
                value: "test".into(),

            }))
        }))

    }
}

