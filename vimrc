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

if has("win32")
  set guifont=Consolas:h10
else
  set guifont=Monospace\ 11
endif

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

"" don't ask for reload of localvimrc
let g:localvimrc_ask=0

"" function to toggle numbering between absolute and relative
function! NumberToggle()
  if(&relativenumber == 1)
    set number
  else
    set relativenumber
  endif
endfunc

"" bind F1 to toggle
nnoremap <F1> :call NumberToggle()<cr>
:au FocusLost * :set number
autocmd InsertEnter * :set number
" automaticall set absolute line numbers when opening a document
autocmd BufNewFile * :set number
autocmd BufReadPost * :set number
autocmd FilterReadPost * :set number
autocmd FileReadPost * :set number


"" fold javadoc
set foldmethod=syntax
set foldenable
autocmd FileType java :set fmr=/**,*/ fdm=marker fdc=1 
autocmd FileType cpp :set fmr=/**,*/ fdm=marker fdc=1
autocmd FileType c :set fmr=/**,*/ fdm=marker fdc=1

" complete options (disable preview scratch window)
set completeopt=menu,menuone,longest

" limit complete popup height
set pumheight=15
