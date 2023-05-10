# What does it do?
It's a simple discord bot server that replies to a `/maths` command like so: 
It is basically a wrapper for [the shunting api package](https://crates.io/crates/shunting).
</br>
![example of command usage (/maths 1+2+3)](command1.png)
![example of command respone (6)](command1b.png)

# Setup
Make sure you have cargo installed. If not, follow the [cargo installation instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html).
Clone this repository. Make sure you are inside the folder with the `README.md` for these last parts.
Create a file `discord.txt` and inside it, paste in your discord bot token. Make sure there are no extra spaces or text - It must be exact.
Run `cargo run`. Cargo may take a long time to download dependencies but once that's done, the bot will start and you will see `<your bot name> is connected`.

# Additional notes:
The following opperations are supported:
- Addition
- Subtraction
- Multiplication
- Division
- Negative numbers
- Powers/exponents (use: `^`)
- Factorial (use: `!`)
- Sin (use: `sin(number)` )
- Cosine (use: `cos(number)`)
- Min (use: `min(number1, number2)`)
- Max (use: max(number1, number2))
- Square root (use: `sqrt(number)`)
- Modulo (use: `%`)
- Absolute magnitute (use: `abs(number)`)
- Natural log (use `log(number)`)

The following cosntants are builtin 
- Pi (use: `pi`)
- E (use: `e`)
