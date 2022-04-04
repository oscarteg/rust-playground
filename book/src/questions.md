# Questions

# How can I separate my commands into separate files?
This was for my Discord bot. I was using a macro to create commands, but I wanted to separate the commands into separate files. The idiot that I am I didn't know how. Adding them into a separate file gave a build error that it could not be found. 

Maybe I can do without the macro but I wanted it to work. The answer someone gave on my question was.

```rust
// you must import static variables the #[command] macro generates. you just need the <COMMAND_FUNCTION_NAME_IN_UPPERCASE>_COMMAND variable. for instance, if you have:
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
// ...
}

// in a module named commands, then you would import the variable with:
use commands::PING_COMMAND;

// when defining commands in a group, the #[group] macro will pick up the variable if you pass in ping to the #[commands(...)] list
```
So separate the commands into a module and static import the commands. Sounds doable.  
