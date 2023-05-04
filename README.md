# plex-tag-automation

Rust binary that allows you to automate tasks by setting tags on plex items.

I created this to automate my music backlog - after listening to an item, setting a tag either moves the album to my main plex music library or deletes it.

Available tasks:

1.  `move` the parent folder of an item to a different folder.
2.  `delete` the parent folder of an item from the file system.

> **Warning**  
> This was created for personal use and tailored to the specifics of my workflow, mainly music albums. Not a lot of work went into making this user-friendly and safe so far.

## Motivation

While a convenient way to script this already exists with the [Plex API](https://github.com/pkkid/python-plexapi), the API requires IP acccess to the instance that Plex is hosted on and stored Plex credentials. This is not easy to setup for some deployment configurations.

This script does _not_ require API access. It works by reading from Plex' sqlite database directly, so it can be hosted alongside Plex, for example as a cron job.

## Setup

1.  Clone this repo to the target server.
2.  Run `cargo build --release`.
3.  Create an `.env` from `.env.example` and adjust `DATABASE_URL`.
4.  Create a `config.yml` file from `config.example.yml` and adjust rules.
5.  Run with `./target/debug/plex-tag-automation`
