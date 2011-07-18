#!/bin/bash
# create a link from .vim/vimrc from repo to 
# .vimrc that VIM expects (same for gvim if necessary)
ln -s ~/.vim/vimrc ~/.vimrc

# update submodules
git submodule update --init
