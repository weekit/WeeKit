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
		-rpath "@executable_path/../dylibs" \
		-o Demo.app/Contents/MacOS/Demo

clean:
	rm -rf Demo.app
