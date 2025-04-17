all:
	@cargo build --release
ifeq ($(OS),Windows_NT)
	@printf "\033[34mto run the program, type: ./target/release/matrix.exe\033[0m\n"
else
	@printf "\033[34mto run the program, type: ./target/release/matrix\033[0m\n"
endif

clean:
	@cargo clean

fclean: clean
	@cargo clean
	@rm -rf target/release/matrix.exe
	@rm -rf target/release/matrix

re: fclean all

.PHONY: all clean fclean re