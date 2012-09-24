filetype off
call pathogen#helptags()
call pathogen#runtime_append_all_bundles()

set hidden
set number
set tabstop=4
set smarttab
set shiftwidth=4
set autoindent
set expandtab
syntax on

set encoding=utf-8

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
set whichwrap+=<,>,h,l

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
set nowb
set noswapfile

" Making it so ; works like : for commands. Saves typing and eliminates :W style typos due to lazy holding shift.
nnoremap ; :

" mapping for Gundo
nnoremap <F5> :GundoToggle<CR>

" Turn on brief-mode for JavaScript indenter
let g:SimpleJsIndenter_BriefMode = 1

" Toggle spelling with \ss
nmap <silent> <leader>ss :setlocal spell!<CR>
set spelllang=en_us

" spelling shortcuts
map <leader>sn ]s
map <leader>sp [s
map <leader>sa zg
map <leader>s? z=

" matching parens key maps
nmap <silent> <leader>k v%
nmap <silent> <ESC><C-K> v%x

" haskellmode-vim
au Bufenter *.hs compiler ghc
if has("win32")
else
    let g:haddock_browser = "/usr/bin/chromium-browser"
endif

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

" auto read when file changed elsewhere
set autoread

" fast saving with \w
nmap <leader>w :w!<cr>

" ignore compiled files
set wildignore=*.o,*~,*.hi

" always show location
set ruler

" ignore case when searching, but be smart about it
set ignorecase
set smartcase

" don't redraw while executing macros
set lazyredraw

set showmatch

" disable annoying sounds
set noerrorbells
set novisualbell
set t_vb=
set tm=500

" smart indent
set ai
set si

" Map space to search and c-space to backwards search
map <space> /
map <c-space> ?

" use \cd to change directory of the current file
map <leader>cd :cd %:p:h<cr>:pwd<cr>

" toggle paste mode
map <leader>pp :setlocal paste!<cr>

let g:SuperTabDefaultCompletionType = "context"

"" javacomplete
if has("autocmd")
  autocmd Filetype haskell setlocal omnifunc=necoghc#omnifunc 
endif
