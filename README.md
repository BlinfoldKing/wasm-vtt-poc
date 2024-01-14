# Ouroboros VTT

a (mostly) hackable and (hopefully) game agnostic virtual table top

## Features

**work in progress**
![WIP](https://media1.tenor.com/m/DaSh5T93TgUAAAAC/cat-typing.gif)

## Development Setup

requirement:

- rust >= 1.75.0
- nodejs >= v21.5.0
- [just](https://github.com/casey/just)

### First Time Building Step

1. for first time setup run: `just install`
2. copy the `build.sample.env` as `build.env`
3. build the app: `just build`

### First Time Running Step

1. copy the `ouroboros.sample.toml` as `ouroboros.toml`
2. build and run the app: `just run`
