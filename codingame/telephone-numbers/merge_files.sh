#!/bin/bash

## TODO Create a gradle plugin to do this ?

## enable powerful pattern matching
shopt -s extglob

cd src/main/java

## print project url
{ echo "///  COMPLETE PROJECT HERE https://github.com/bubuntux/CodinGame  ///" && \
## print imports
cat *.java | grep import | sort | uniq && \
## print Player class without the last '}'
cat Player.java | grep -v import | sed '/^$/d' | head -n -1 && \
## print other classes
cat -- !(Player).java | grep -v import | sed '/^$/d' && \
## print '}' to close player class
echo '}' ;} | \
## put everything into the clipboard
xclip -selection c