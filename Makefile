export PATH := $(HOME)/.local/bin:$(PATH)

all: make_localbin build install

install:
	cp target/release/askme-* $(HOME)/.local/bin/
	rm -f $(HOME)/.local/bin/askme-*.d

build:
	cargo build --release

make_localbin:
	$(shell if [[ ! -d "$(HOME)/.local/bin/" ]]; then \
		mkdir $(HOME)/.local/bin; \
	fi)

clean:
	for dir in target/release target/debug ; do \
        if [ -d "$dir" ]; then \
            rm "$$dir" || exit 1; \
        fi \
    done

uninstall:
	rm $(HOME)/.local/bin/askme-*;