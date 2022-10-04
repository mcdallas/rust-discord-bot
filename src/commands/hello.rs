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
        let name = _input.get_option("name").unwrap_or("World");
        
        Ok(InteractionApplicationCommandCallbackData {
            content: Some(format!("Hello, {}!", name)),
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
            name: "name".into(), 
            autocomplete: Some(true), 
            description: "Your name".into(), 
            required: Some(false), 
            ty: ApplicationCommandOptionType::String,
            choices: None,
        }])
    }

    async fn autocomplete(&self,  _input: &CommandInput) -> Result<Option<InteractionApplicationCommandCallbackData>, InteractionError> {
        Ok(Some(InteractionApplicationCommandCallbackData {
            content: None,
            embeds: None,
            choices: Some(vec![ApplicationCommandOptionChoice{
                name: "Alice".into(),
                value: "Alice".into(),
            },
            ApplicationCommandOptionChoice{
                name: "Bob".into(),
                value: "Bob".into(),
            },
            ApplicationCommandOptionChoice{
                name: "Charlie".into(),
                value: "Charlie".into(),
            }])
        }))
    }
}

