.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports \
	clippy \
	crit \
	doc \
	docker-build \
	docker-build-alpine \
	docker-build-debian \
	docker-push \
	docker-push-alpine \
	docker-push-debian \
	install \
	lint \
	port \
	publish \
	rustfmt \
	test \
	uninstall
.IGNORE: \
	clean \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports

VERSION=0.0.25
BANNER=tuggy-$(VERSION)

all: build

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports

clean-archive:
	rm ".crit/bin/$(BANNER).tgz"

clean-cargo:
	cargo clean

clean-example:
	rm -f example/Cargo.lock
	rm -rf example/target
	rm -rf example/.crit

clean-ports:
	crit -c

clippy:
	cargo clippy

CRIT_EXCLUSIONS=android|cuda|emscripten|fortanix|fuchsia|gnullvm|gnux32|ios|loongarch|msvc|none-eabi|ohos|pc-solaris|powerpc64le-unknown-linux-musl|redox|riscv64gc-unknown-linux-musl|sparcv9-sun-solaris|uefi|unknown-none|wasm|i686-pc-windows-gnu

crit:
	crit -b $(BANNER) -e "$(CRIT_EXCLUSIONS)"

doc:
	cargo doc

docker-build: docker-build-alpine docker-build-debian

docker-build-alpine:
	tuggy \
		-c tuggy.alpine.toml \
		-t "n4jm4/tuggy:$(VERSION)-alpine3.23" \
		--load

docker-build-debian:
	tuggy \
		-c tuggy.debian.toml \
		-t "n4jm4/tuggy:$(VERSION)-trixie" \
		--load

docker-push: docker-push-alpine docker-push-debian

docker-push-alpine:
	tuggy \
		-c tuggy.alpine.toml \
		-t "n4jm4/tuggy:$(VERSION)-alpine3.23" \
		-a "n4jm4/tuggy:alpine3.23" \
		--push

docker-push-debian:
	tuggy \
		-c tuggy.debian.toml \
		-t "n4jm4/tuggy:$(VERSION)-trixie" \
		-a "n4jm4/tuggy:trixie,n4jm4/tuggy:$(VERSION),n4jm4/tuggy" \
		--push

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

port: crit
	chandler -C .crit/bin -czf "$(BANNER).tgz" "$(BANNER)"

publish:
	cargo publish

rustfmt:
	cargo fmt

test:
	cargo test

uninstall:
	cargo uninstall tuggy
