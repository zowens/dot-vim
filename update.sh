#!/bin/bash
git pull origin master
env GIT_SSL_NO_VERIFY=true git submodule update --init
env GIT_SSL_NO_VERIFY=true git submodule foreach git pull origin master

# updates gocode
go get -u github.com/nsf/gocode
git -C ~/src/gocode pull origin master
cp -r ~/src/gocode/vim/autoload bundle/gocode
cp -r ~/src/gocode/vim/ftplugin bundle/gocode

git add bundle/
