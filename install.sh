#!/bin/bash

GIT_SSL_NO_VERIFY=true

if [ -d "~/.vim" ]; then
  echo "Moving old vim files from ~/.vim to ~/vim-old"
  mv ~/.vim ~/vim-old
  rmdir ~/.vim
fi

git clone http://github.com/zowens/dot-vim.git ~/.vim

# create a link from .vim/vimrc from repo to 
# .vimrc that VIM expects (same for gvim if necessary)
ln -s ~/.vim/vimrc ~/.vimrc

# update submodules
cd ~/.vim
git submodule update --init
