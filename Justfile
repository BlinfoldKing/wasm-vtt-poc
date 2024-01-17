set dotenv-filename := "build.env"

env := env_var_or_default("ENVIRONMENT", "dev")
is-dev := if env == "dev" { "true" } else if env == "development" { "true" } else { "false" }
is-prod := if env == "prod" { "true" } else if env == "production" { "true" } else { "false" }

install:
	@echo "installing yarn..."
	@npm i -g yarn
	@echo "yarn install..."
	@npm i
	@echo "install watch..."
	@cargo install cargo-watch
	@just build
	@npm i concurrently

about:
	@echo {{ env }}

build:
	@just build-pkgs
	@yarn {{ if is-dev == "true" {"build-dev"} else {"build"} }}
	@just build-server

build-pkgs:
	@echo build engine on {{env}}
	@wasm-pack build packages/vtt --target web {{ if is-dev == "true" { "--dev" } else { "--release" } }}

build-server:
	@echo build server on {{env}}
	@cargo build --package server --profile {{ if is-dev == "true" { "dev" } else { "release" } }}

build-docker:
	@just build
	@cp target/{{ if is-dev == "true" {"debug"} else { "release" } }}/server dist/server
	@docker build -t ouroboros . --progress=plain

run: 
	@just build
	@rm -rf vite.config.ts.timestamp*
	@cargo run --package server -- setup
	@concurrently "just http" "just browser"

http:
	@cargo run --package server -- http

browser: 
	@open http://localhost:8080

dev:
	@cargo watch -s 'just run' -i "dist/" -i "target/"
