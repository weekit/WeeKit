#!/bin/sh

#
# Format all Markdown files in this repository using prettier.
# (https://prettier.io/)
#

for f in `git ls-files | egrep '.md$'` 
do
	prettier --write --print-width 80 --prose-wrap always $f
done
