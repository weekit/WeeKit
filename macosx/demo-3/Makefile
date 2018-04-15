#!/bin/sh

SDK=~/Desktop/amanithvg-sdk

all:	
	mkdir -p Demo.app/Contents/MacOS
	mkdir -p Demo.app/Contents/dylibs
	mkdir -p Demo.app/Contents/Resources
	cp resources/Info.plist Demo.app/Contents
	cp resources/tutorial.icns Demo.app/Contents/Resources
	cp ${SDK}/lib/macosx/ub/gle/standalone/libAmanithVG.4.dylib Demo.app/Contents/dylibs
	clang src/*.m src/*.c \
		-g \
		-I ${SDK}/include \
		-L ${SDK}/lib/macosx/ub/gle/standalone \
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
		-o libapp.dylib
	install_name_tool -id "@rpath/../dylibs/libapp.dylib" libapp.dylib 
	install_name_tool -add_rpath "@executable_path/../dylibs/" libapp.dylib 
	mv libapp.dylib Demo.app/Contents/dylibs/libapp.dylib

	cd rust; cargo build
	cp rust/target/debug/demo Demo.app/Contents/MacOS/Demo
	install_name_tool -add_rpath "@executable_path/../dylibs" Demo.app/Contents/MacOS/Demo

clean:
	rm -rf Demo.app
	cd rust; cargo clean
