all: add_localbin_to_path build
	cp -i target/release/askme-* $(HOME)/.local/bin/

install:
	cp -i target/release/askme-* $(HOME)/.local/bin/

build:
	cargo build --release

add_localbin_to_path:
	$(shell if [[ ! -d "$(HOME)/.local/bin/" ]]; then \
		mkdir $(HOME)/.local/bin; \
	fi)

	$(shell export PATH="$PATH:$HOME/.local/bin")
	@echo "$(HOME)/.local/bin was added to the PATH in this session."
	@echo "to make this change global, please do so in you shell's rc."

clean:
	rm -rvf target/release;

uninstall:
	rm $(HOME)/.local/bin/askme-*;
