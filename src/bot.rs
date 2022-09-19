use crate::interaction::{InteractionResponse, Interaction};
use crate::http::HttpError;
use crate::error::Error;
use crate::verification::verify_signature;
use worker::{Request, RouteContext};


pub struct App {
    req: Request, 
    ctx: RouteContext<()>
}

impl App {

    pub fn new(req: Request, ctx: RouteContext<()>) -> App {
        App{req, ctx}
    }

    fn var(&self, key: &str) -> Result<String, Error> {
        return match self.ctx.var(key) {
            Ok(var) =>  Ok(var.to_string()),
            Err(_) =>  Err(Error::EnvironmentVariableNotFound(key.to_string()))
        }

    }
    fn header(&self, key:&str) -> Result<String, Error> {
        return match  self.req.headers().get(key) {
            Ok(val) => val.ok_or_else(|| Error::HeaderNotFound(key.to_string())),
            Err(_) => Err(Error::HeaderNotFound(key.to_string()))
        }
    }

    async fn validate_sig(&mut self) -> Result<String, Error> {
        let pubkey = self.var("DISCORD_PUBLIC_KEY")?;
        let signature = self.header("x-signature-ed25519")?;
        let timestamp = self.header("x-signature-timestamp")?;

        let body = self.req.text().await.map_err(|_| Error::InvalidPayload("".into()))?;
        verify_signature(&pubkey, &signature, &timestamp, &body).map_err(Error::VerificationFailed)?;
        return Ok(body)
    }

    pub async fn handle_request(&mut self) -> Result<InteractionResponse, HttpError> {
        let body = self.validate_sig().await?;

        worker::console_log!("Request body : {}", body);
        
        let interaction =
        serde_json::from_str::<Interaction>(&body).map_err(Error::JsonFailed)?;
        worker::console_log!{"Request parsed : {}", serde_json::to_string_pretty(&interaction).unwrap()};
        let response = interaction.perform(&mut self.ctx).await?;
        
        Ok(response)

    }

}