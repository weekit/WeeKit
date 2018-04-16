#!/bin/sh

all:	
	# Build libwee.
	clang src/*.c \
		-g \
		-I /opt/vc/include \
		-I /usr/local/include \
		-L /usr/local/lib \
		-I src \
		-lobjc \
		-shared -fpic \
		-ljpeg \
		-o libwee.so

	# Build and install the main (Rust) binary.
	cd rust; cargo build

clean:
	cd rust; cargo clean
	rm -rf libwee.so
