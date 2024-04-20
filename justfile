default:
    just --list

watch:
    cargo watch -c --ignore output -x 'r -- --openapi-path examples/openapi.yaml'

watch-check:
    cargo watch -c --ignore output -x c

dev:
    cargo r -- --openapi-path examples/openapi.yaml
