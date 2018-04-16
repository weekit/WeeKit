#!/bin/sh

SDK=~/Desktop/amanithvg-sdk

all:	
	# Build libwee.
	clang src/*.m src/*.c \
		-g \
		-I ${SDK}/include \
		-L ${SDK}/lib/macosx/ub/gle/standalone \
		-I /usr/local/include \
		-L /usr/local/lib \
		-lAmanithVG \
		-I src \
		-lobjc \
		-framework Cocoa \
		-framework CoreVideo \
		-framework CoreFoundation \
		-framework QuartzCore \
		-framework OpenGL \
		-shared -fpic \
		-ljpeg \
		-rpath "@executable_path/../dylibs" \
		-o libwee.dylib
	# Build the .app bundle.
	mkdir -p Demo.app/Contents/MacOS
	mkdir -p Demo.app/Contents/dylibs
	mkdir -p Demo.app/Contents/Resources
	cp resources/Info.plist Demo.app/Contents
	cp resources/tutorial.icns Demo.app/Contents/Resources
	cp ${SDK}/lib/macosx/ub/gle/standalone/libAmanithVG.4.dylib Demo.app/Contents/dylibs
	install_name_tool -id "@rpath/../dylibs/libwee.dylib" libwee.dylib 
	install_name_tool -add_rpath "@executable_path/../dylibs/" libwee.dylib 
	mv libwee.dylib Demo.app/Contents/dylibs/libwee.dylib
	# Build and install the main (Rust) binary.
	cd rust; cargo build
	cp rust/target/debug/demo Demo.app/Contents/MacOS/Demo
	install_name_tool -add_rpath "@executable_path/../dylibs" Demo.app/Contents/MacOS/Demo

clean:
	cd rust; cargo clean
	rm -rf Demo.app *.dSYM
