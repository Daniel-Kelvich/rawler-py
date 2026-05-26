.PHONY: dev build clean test

dev:
	maturin develop --release

build:
	maturin build --release

test: dev
	python3 -c "import rawler_py; print(dir(rawler_py.RawImage))"

clean:
	cargo clean
