.PHONY: clean

CLEAN_DIRNAMES := target/debug target/release
CLEAN_DIRS := $(strip $(foreach dir,$(CLEAN_DIRNAMES),$(wildcard $(dir))))

export PATH := $(HOME)/.local/bin:$(PATH)

# fix for missing headers
# (assumes that the headers were installed via brew)
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	ifeq ($(PROCESSOR_ARCHITECTURE),x86_64)
		export CPATH := /usr/local/include
		export LIBRARY_PATH := /usr/local/lib
	else ifeq ($(PROCESSOR_ARCHITECTURE),arm64)
		export CPATH := /opt/homebrew/include
		export LIBRARY_PATH := /opt/homebrew/lib
	endif
endif

all: make_localbin build install

install:
	cp target/release/askme-* $(HOME)/.local/bin/
	rm -f $(HOME)/.local/bin/askme-*.d

build:
	cargo build --release

make_localbin:
	[ -d $(HOME)/.local/bin ] || mkdir -p $(HOME)/.local/bin

clean: 
ifneq (,$(CLEAN_DIRS))
	rm -rv $(CLEAN_DIRS)
endif

uninstall:
	