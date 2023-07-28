# Rust Discord Framework (Working Title)
This is a framework I'm developing to make writing commands for integration with [serenity.rs](https://github.com/serenity-rs/serenity) a whole lot easier.

The framework is heavily inspired and at times guided by [poise](https://github.com/serenity-rs/poise) an already existing framework written for [serenity](https://github.com/serenity-rs/serenity) too.

Why reinvent the wheel, you ask? Well...

I felt that, even though, [poise](https://github.com/serenity-rs/poise) did in fact simplify the process of writing commands, what it also did was introduce a significant amount of abstraction to the already high level API that [serenity](https://github.com/serenity-rs/serenity) provided. In addition, a lot of the features that poise provided were not really useful to me; such as, the message based commands.

In addition, I recently took interest in [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) in Rust. They have proven to be a quite powerful metaprogramming tool, and given the fact that [poise](https://github.com/serenity-rs/poise) also uses them to reduce boilerplate code, I felt that recreating a similar library to be used with my already in-development Discord bot would be an interesting learning experience.

TL;DR: A [proc-macro](https://doc.rust-lang.org/reference/procedural-macros.html) based framework for [serenity.rs](https://github.com/serenity-rs/serenity) (a Discord API Library) to simplify the task of writing commands by reducing boilerplate code and minimizing abstraction. Heavily inspired by [poise](https://github.com/serenity-rs/poise).