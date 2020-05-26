#!/bin/bash

## TODO Create a gradle plugin to do this ?
cd src/main/java

## print project url
{ echo "///  COMPLETE PROJECT HERE https://github.com/bubuntux/CodinGame  ///" && \
## print imports
find . -type f -name "*.java" -exec cat {} + | grep import | grep -v org.bubuntux | sort | uniq && \
## print Player class without the last '}'
find . -type f -name "Player.java" -exec cat {} + | sed '/^$/d' | grep -v package | grep -v import | head -n -1 && \
## print other classes
find . -type f \( -name "*.java" ! -name "Player.java" \) -exec cat {} + | sed '/^$/d' | grep -v package | grep -v import && \
## print '}' to close player class
echo '}' ;} | \
## put everything into the clipboard
xclip -selection c