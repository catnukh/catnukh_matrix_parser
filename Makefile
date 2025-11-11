.PHONY: all run test fmt lint pre-commit

examples/add.txt:
    @mkdir -p examples
    @echo "# Test addition" > examples/add.txt
    @echo "mat A = [[1,2],[3,4]]" >> examples/add.txt
    @echo "mat B = [[5,6],[7,8]]" >> examples/add.txt
    @echo "add A, B" >> examples/add.txt

examples/sub.txt:
    @mkdir -p examples
    @echo "# Test subtraction" > examples/sub.txt
    @echo "mat B = [[5,6],[7,8]]" >> examples/sub.txt
    @echo "mat my_third_matrix = [[9,10],[11,12]]" >> examples/sub.txt
    @echo "sub B, my_third_matrix" >> examples/sub.txt

examples/mul.txt:
    @mkdir -p examples
    @echo "# Test multiplication" > examples/mul.txt
    @echo "mat A = [[1,2],[3,4]]" >> examples/mul.txt
    @echo "mat my_third_matrix = [[9,10],[11,12]]" >> examples/mul.txt
    @echo "mul A, my_third_matrix" >> examples/mul.txt

examples/scale.txt:
    @mkdir -p examples
    @echo "# Test scale" > examples/scale.txt
    @echo "mat A = [[1,2],[3,4]]" >> examples/scale.txt
    @echo "scale A, 3.4" >> examples/scale.txt

run-add: examples/add.txt
    @echo "---Run addition---"
    cargo run -- parse examples/add.txt

run-sub: examples/sub.txt
    @echo "---Run subtraction---"
    cargo run -- parse examples/sub.txt

run-mul: examples/mul.txt
    @echo "---Run multiplication---"
    cargo run -- parse examples/mul.txt

run-scale: examples/scale.txt
    @echo "---Run scale---"
    cargo run -- parse examples/scale.txt

test:
	@echo "---Start tests---"
	cargo test

fmt:
	@echo "---Formating---"
	cargo fmt

lint:
	@echo "---Linter(cargo clipy)---"
	cargo clipy -- -D warnings

pre-commit: fmt lint test
	@echo "Tests complited"
