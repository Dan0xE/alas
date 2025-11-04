# ALAS - Alias Reminder Plugin for Nushell

A quickly hacked togehter nushell plugin for the lazy folks that reminds you to use your defined aliases instead of typing full commands.

> [!NOTE]
> Disclaimer: I am by no means a nushell expert - If you have any suggestions or improvements, please open an issue or a PR!

## How That Thing Works

When you run a command, the plugin checks if an alias exists for it and reminds you to use the alias.

### Example

Let's say you have these aliases defined:

```nushell
alias d = docker
alias dc = docker compose
```

- When you type `docker ps`, you'll see: `Alias 'd' exists for 'docker'`.
- When you type `docker compose up`, you'll see: ` Alias 'dc' exists for 'docker compose'` as the more "specific" alias takes precedence.

## Quick Start

1. Build the plugin:
   ```bash
   cargo build --release
   ```

2. Register it with Nushell:
   ```nushell
   plugin add target/release/alas
   ```

3. Add the pre-execution hook to your config:
   ```nushell
   $env.config.hooks.pre_execution = [{||
       let cmd = (commandline) | str trim
       let result = alas $cmd (scope aliases)
       if ($result | str length) > 0 {
           print $"(ansi yellow)($result)(ansi reset)"
       }
   }]
   ```

## Requirements

- Nushell v0.108
- Rust toolchain (for building)
