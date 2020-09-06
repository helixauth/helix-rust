SHELL := /bin/bash

# Make the repo
# all: clean mock test

dev: 
	docker-compose up -d ; \
	cargo watch -x run

build:
	cargo build
