CARGO = cargo
TARGET_DIR = target
BIN_NAME = my_cli


clean:
	$(CARGO) clean


build:
	$(CARGO) build


run: build
	@./target/debug/my_cli 


greet: build
	@./target/debug/my_cli greet -n John -s Doe 


goodbye: build
	@./target/debug/my_cli goodbye -n John -s Doe


test:
	$(CARGO) test


help: build
	@./target/debug/my_cli help

