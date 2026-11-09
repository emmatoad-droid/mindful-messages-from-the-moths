# 🦋 Mindful Messages from the Moths

A tiny Rust programme delivering unexpected messages of kindness, one moth at a time.

## What is this?

Somewhere between a library shelf, a little piece of poetry and a curious encounter with a moth, this project came into existence.

Mindful Messages from the Moths is a small Rust command-line programme that randomly selects a moth from a collection of eight, displays its ASCII illustration and delivers an accompanying message.

Each run brings a different possibility.

A little reminder that somebody, somewhere, might need a kind word today.

## The moth collection

Eight moths. Eight illustrations. Eight messages.

| Moth | Message |
| --- | --- |
| Luna Moth | Begin softly. You don’t need to glow loudly to be seen. |
| Atlas Moth | You are bigger than the day you’re having. Take up your space. |
| Peppered Moth | Changing isn’t losing yourself. Sometimes it’s how you survive. |
| Death’s-head Hawkmoth | Nothing lasts forever, so notice this moment. It’s precious. |
| Garden Tiger Moth | Your boundaries are allowed to be loud. |
| Emperor Moth | Pause. Breathe in the weather you’re in, even the wild bits. |
| White Ermine | Rest is allowed. Breathe out and soften. |
| Lantern Moth | Keep turning towards the light, even when you’re tired. |

## Features

- Eight moth and message pairings.
- Individual ASCII moth illustrations.
- Randomised selection on each execution.
- Lightweight, self-contained Rust implementation.
- No external crate dependencies.
- Terminal-based output.

## Running the programme

You’ll need Rust and Cargo installed.

Clone the repository, navigate into the project directory and run:

```sh
cargo run
```

Each execution selects a moth and displays its illustration and message.

## Under the wings

The project uses:

- Rust structs to represent moths and their attributes.
- A constant array to hold the moth collection.
- A selection function to choose a moth.
- System time as the seed for a lightweight pseudo-random selection routine.
- Standard output to display the result.

## Why moths?

Moths are often associated with transformation, resilience, vulnerability and the search for light.

This project takes those associations and turns them into small, unexpected messages.

Sometimes a message arrives exactly when you need it.

Sometimes you just get a moth.

Both are perfectly acceptable.

---

Built with Rust, curiosity, poetry and a fondness for things that flutter.

Turn towards the light, little moth. 🦋
