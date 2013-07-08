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
set background=dark
set listchars=tab:▸\ ,eol:¬

if has("win32")
  set guifont=Consolas\ for\ Powerline\ FixedD:h9
  "disable loading perforce in tier
  if !exists("$TIER")
      let loaded_perforce=1
  endif
else
  set guifont=Inconsolata\-dz\ for\ Powerline:h14
  " supress loading of perforce plugin
  let loaded_perforce=1
endif

let g:Powerline_symbols="fancy"

" exclusively use VIM settings (not VI settings)
set nocompatible

set laststatus=2

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

" Toggle spelling with \ss
nmap <silent> <leader>ss :setlocal spell!<CR>
set spelllang=en_us

" spelling shortcuts
map <leader>sn ]s
map <leader>sp [s
map <leader>sa zg
map <leader>sd zG
map <leader>s? z=

" matching parens key maps
nmap <silent> <leader>k v%
nmap <silent> <ESC><C-K> v%x

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
nnoremap <F2> :call NumberToggle()<cr>
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
set tm=1000 " timeout to 1 second

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

nmap <F8> :TagbarToggle<CR>

let g:ctrlp_map = '<c-t>'

" remove trailing whitespace on save
nmap <F7> :%s/\s\+$//e<cr>

let g:airline_enable_syntastic=1
let g:airline_enable_fugitive=1
