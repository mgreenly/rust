.DEFAULT_GOAL := all

all: $(patsubst %.rs, %, $(wildcard *.rs))

%: %.rs
	rustc -o $@ $<

.PHONY: clean
clean:
	find . -maxdepth 1 -type f -executable -exec rm {} +
