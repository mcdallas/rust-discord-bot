
# About

A pure-Rust serverless discord chatbot hosted on Cloudflare Workers. With a free account you have up to 100k requests per day. For storing state you can use the bundled [`workers-rs`](https://github.com/cloudflare/workers-rs) crate to access KV or Durable objects.

This template is designed for compiling Rust to WebAssembly and publishing the resulting worker to 
Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).


## Setup

1. Signup for a Cloudflare account, in the dashboard setup a subdomain (i.e `<mydomain>.workers.dev`)
2. Setup a worker project named `bot` (i.e `bot.<mydomain>.workers.dev`) or pick your own name and update wrangler.toml
3. Install [wrangler CLI](https://github.com/cloudflare/wrangler) with `cargo install wrangler` and authenticate with cloudflare via `wrangler config`
4. Create a new discord app at https://discord.com/developers/applications and copy your token/application_id/public_key
5. Pass those secrets to your bot with `wrangler secret put DISCORD_TOKEN`, `wrangler secret put DISCORD_PUBLIC_KEY`, `wrangler secret put DISCORD_APPLICATION_ID`
6. [Add bot permissions](https://discord.com/developers/docs/tutorials/hosting-on-cloudflare-workers#adding-bot-permissions) and grab your Oauth url to invite the bot to your server
7. Publish the demo app with `wrangler publish`. The template bot contains a single hello command with a dummy autocomplete argument.
8. Put your bot domain `https://bot.<mydomain>.workers.dev` in the `INTERACTIONS ENDPOINT URL` in your discord app page from step 4
9. After initial deployment and each time you add a new command on your bot you need to register it with the discord api. To do that simply `curl -X POST http://bot.<mydomain>.workers.dev/register`

You should now be able to run the `/hello` command on discord 


## Adding new commands

To add a new command simply implement the `Command` trait. For example to add a ping command

1. create a file src/commands/ping.rs

``` rust
use crate::interaction::{
    InteractionApplicationCommandCallbackData, ApplicationCommandOption, ApplicationCommandOptionChoice, ApplicationCommandInteractionDataOption, ApplicationCommandOptionType
};
use crate::error::InteractionError;
use crate::command::Command;

use async_trait::async_trait;


pub(crate) struct Ping {}

#[async_trait(?Send)]
impl Command for Ping {
    async fn respond(&self, _options: &Option<Vec<ApplicationCommandInteractionDataOption>>, _ctx: &mut worker::RouteContext<()>) -> Result<InteractionApplicationCommandCallbackData, InteractionError>{
        Ok(InteractionApplicationCommandCallbackData {
            content: Some("Pong".to_string()),
            choices: None,
            embeds: None
        })
    }

    fn name(&self) -> String{
        "ping".into()
    }

    fn description(&self) -> String {
        "Send a ping".into()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        // add any arguments/choices here, more info at https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
        None
    }

    async fn autocomplete(&self, _options: &Option<Vec<ApplicationCommandInteractionDataOption>>, _ctx: &mut worker::RouteContext<()>) -> 
        None
    }

```
2. add your new module in src/commands/mod.rs
3. Register your command in  `init_commands` in src/command.rs 
``` rust
pub(crate) fn init_commands() -> Vec<Box<dyn Command + Sync>> {
    let mut v : Vec<Box<dyn Command + Sync>> = Vec::new();
    v.push(Box::new(commands::hello::Hello {}));
    // Add this line
    v.push(Box::new(commands::ping::Ping {}));
    v
}
```
4. publish your package with `wrangler publish`
5. register your new command with discord with `curl -X POST http://bot.<mydomain>.workers.dev/register`

You can store and access state using the `ctx` context object passed to the `respond` and `autocomplete` methods, for example:

``` rust
let kv = ctx.kv("my_namespace")?;  // the namespace must be first registered on cloudflare dashboard
let my_val =  kv.get("my_key").text().await?;
kv.put("foo", "bar")?.execute().await?;

```

## Local Dev 


With `wrangler`, you can build, test, and deploy your Worker with the following commands: 

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```

you can use `ngrok` to tunnel traffic into your local machine, more info [here](https://discord.com/developers/docs/tutorials/hosting-on-cloudflare-workers#setting-up-ngrok)

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as 
compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and
modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple. 

Read more about this on the [`workers-rs` project README](https://github.com/cloudflare/workers-rs).

## Credits

based on [stateless-discord-bot](https://github.com/siketyan/stateless-discord-bot)