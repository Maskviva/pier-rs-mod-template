# pier-rs-mod-template

A starting point for writing [LeviLamina](https://github.com/LiteLDev/LeviLamina) mods
for Minecraft Bedrock Dedicated Server **in Rust**, with no C++ toolchain.

This template targets [**Pier**](https://github.com/Maskviva/pier), the loader that
teaches LeviLamina to load mods written in other languages. Install Pier on the server
first; without it this mod is never scanned.

[中文说明](README.zh.md)

## Getting started

1. Press **Use this template** above, then clone the new repository.
2. Rename three things so they agree with each other:
   - `Cargo.toml` → `[package] name`
   - `manifest.json` → `name`, which must match the directory you install into, and
     `entry`, which is the DLL cargo produces. Cargo turns hyphens into underscores, so
     `my-mod` builds `my_mod.dll`.
   - `src/lib.rs` → the `HelloWorld` struct and the strings that mention it
3. Write your mod. Delete whichever of the four demonstrated pieces you do not need.
4. Build and install:

   ```bash
   cargo build --release
   ```

   Copy `target/release/<your_crate>.dll` and `manifest.json` into
   `<server>/mods/<your-mod>/`. The workflows in `.github/` do this on every push and
   attach an archive to every release.

## What the template shows

`src/lib.rs` is a working mod, not a skeleton:

- **Logging** through the mod's own logger, so the console shows `[your-mod]` and not
  `[pier]`.
- **An event subscription that cancels.** Chat messages are logged and anything
  mentioning a creeper is blocked. The subscription is held in the struct, so dropping
  it unsubscribes.
- **A custom command**, `/hello [name]`.
- **A scheduled task** that runs back on the server thread and executes a console
  command.

## Things worth knowing before you start

**`"type": "pier"` in the manifest is what makes the host find the mod.** One wrong
character and the mod is never scanned, with no error reported anywhere.

**Use the constants in `names`, not literal event ids.** `event::subscribe` with a
misspelled id succeeds and then never fires. `names::PLAYER_CHAT` fails at compile time
instead.

**A value that cannot be read comes back as `Err`, never as a default.** An event payload
missing a field is a real error, because treating a missing dimension as the overworld is
how land protection gets bypassed. Handle the `Err`; do not reach for `unwrap_or(0)`.

**Callbacks run on the server thread**, except the packet interception callbacks, which
run on whichever thread pumps the connection. `Host::get().schedule()` and
`schedule_after()` are how a background thread gets work back onto the server thread.

## A minimal mod

```rust
use levilamina::prelude::*;

struct MyMod;

impl LeviMod for MyMod {
    fn on_load(ctx: &ModContext) -> Result<Self> {
        ctx.logger().info("hello from Rust");
        Ok(MyMod)
    }
}

levilamina::register_mod!(MyMod);
```

`on_enable`, `on_disable` and `on_unload` are optional and default to doing nothing.

## Requirements

- Rust, targeting `x86_64-pc-windows-msvc`, which is the default on Windows
- [LeviLamina](https://lamina.levimc.org/) 26.20.4 on BDS 1.26.20
- [Pier](https://github.com/Maskviva/pier) installed on the target server

## Where to ask

A question about this skeleton belongs here. A missing API, or a bug in the loader
itself, belongs in [Pier](https://github.com/Maskviva/pier/issues).

## License

CC0-1.0, public domain. Do whatever you want with the starter code; no attribution
needed.

---

*Not affiliated with Mojang, Microsoft or LeviMC. Minecraft is a trademark of Mojang
Synergies AB.*
