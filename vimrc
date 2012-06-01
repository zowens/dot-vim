filetype off
call pathogen#helptags()
call pathogen#runtime_append_all_bundles()

set hidden
set number
set tabstop=4
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

if has("autocmd")
  filetype plugin indent on
endif

" command mapping from nt to NERDTree
nmap <silent> <leader>nt :NERDTreeToggle<CR>
let NERDTreeIgnore=['\.pyc', '\~$', '\.swo$', '\.swp$', '\.git', '\.hg', '\.svn', '\.bzr']

" Stupid shift key fixes
"cmap W w 						
"cmap WQ wq
"cmap wQ wq
"cmap Q q

" turn off backup files
set nobackup

" Making it so ; works like : for commands. Saves typing and eliminates :W style typos due to lazy holding shift.
nnoremap ; :

" mapping for Gundo
nnoremap <F5> :GundoToggle<CR>

" Turn on brief-mode for JavaScript indenter
let g:SimpleJsIndenter_BriefMode = 1

" Toggle spelling with \s
nmap <silent> <leader>s :set spell!<CR>
set spelllang=en_us

" matching parens key maps
nmap <silent> <leader>k v%
nmap <silent> <ESC><C-K> v%x

" haskellmode-vim
au Bufenter *.hs compiler ghc
let g:haddock_browser = "/usr/bin/chromium-browser"
