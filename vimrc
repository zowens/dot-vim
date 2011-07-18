filetype off
call pathogen#helptags()
call pathogen#runtime_append_all_bundles()

set hidden
set number
set tabstop=2
set smarttab
set shiftwidth=2
set autoindent
set expandtab
syntax on

colorscheme solarized
set background=light
set listchars=tab:▸\ ,eol:¬
set guifont=Monospace\ 11

" exclusively use VIM settings (not VI settings)
set nocompatible

"allow backspacing
set backspace=indent,eol,start

set nowrap
set softtabstop=4
