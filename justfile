default:
    just --list

watch:
    cargo watch -c --ignore output -x r

watch-check:
    cargo watch -c --ignore output -x c
