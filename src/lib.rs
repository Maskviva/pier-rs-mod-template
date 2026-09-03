//! hello-world: a starting point for a LeviLamina mod written in Rust.
//!
//! This is a working mod rather than an empty skeleton. It shows the four things almost
//! every mod needs: logging, an event subscription that can cancel, a custom command,
//! and work scheduled back onto the server thread. Read it, then delete what you do not
//! need.
//!
//! Build with `cargo build --release` and copy `target/release/hello_world.dll` next to
//! `manifest.json` in `<server>/mods/hello-world/`.

use levilamina::command::CommandPermission;
use levilamina::prelude::*;
use levilamina::{command, event};

struct HelloWorld {
    /// Dropping a `Listener` unsubscribes, so holding it here ties the subscription to
    /// the lifetime of the mod. `forget()` gives that up and keeps it until unload.
    chat: Option<Listener>,
}

impl LeviMod for HelloWorld {
    fn on_load(ctx: &ModContext) -> Result<Self> {
        ctx.logger().info("hello-world loaded");
        Ok(HelloWorld { chat: None })
    }

    fn on_enable(&mut self, ctx: &ModContext) -> Result<()> {
        let logger = ctx.logger();
        let host = ctx.host();

        // 1. Events. Use the constants in `names` rather than a literal: a misspelled
        //    event id subscribes successfully and then never fires.
        self.chat = Some(event::subscribe(names::PLAYER_CHAT, |ev| {
            let log = Logger::get();
            let Ok(message) = ev.str_at("message") else {
                // The payload had no message field. Reading it as an empty string
                // would silently let anything through, so this leaves the event alone.
                return;
            };
            let message = message.to_owned();
            log.info(&format!("[chat] {message}"));
            if message.contains("creeper") {
                if let Err(e) = ev.cancel() {
                    log.error(&format!("could not cancel the chat event: {e}"));
                }
            }
        })?);

        // 2. A custom command: /hello [name]
        command::register(
            "hello",
            "Say hello from Rust",
            CommandPermission::Any,
            |inv| {
                let who = if inv.raw().is_empty() {
                    inv.origin().name
                } else {
                    inv.raw().to_owned()
                };
                inv.success(&format!("Hello, {who}!"));
            },
        )?;

        // 3. Scheduling. A callback runs on the server thread, which is where the world
        //    may be touched at all. `Host::get()` is how a background thread gets back
        //    here.
        host.schedule_after(std::time::Duration::from_secs(5), || {
            let log = Logger::get();
            match Host::get().execute_command("say hello-world is alive") {
                Ok(out) => log.info(&format!("broadcast: {out}")),
                Err(e) => log.error(&format!("broadcast failed: {e}")),
            }
        })?;

        logger.info("hello-world enabled, try /hello or say something in chat");
        Ok(())
    }

    fn on_disable(&mut self, ctx: &ModContext) -> Result<()> {
        // Dropping the listener unsubscribes. A command cannot be deregistered on
        // Bedrock, so the host mutes its callback instead and resumes on re-enable.
        self.chat = None;
        ctx.logger().info("hello-world disabled");
        Ok(())
    }
}

levilamina::register_mod!(HelloWorld);
