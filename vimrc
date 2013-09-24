filetype off
" standard pathogen stuff
call pathogen#helptags()
call pathogen#runtime_append_all_bundles()

" standard vim editing config, 4 space tabs
set hidden
set number
set tabstop=4
set smarttab
set shiftwidth=4
set autoindent
set expandtab

syntax on

" UTF-8 always
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
  set guifont=Inconsolata\ for\ Powerline:h16
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

let NERDTreeIgnore=['\.pyc', '\~$', '\.swo$', '\.swp$', '\.git', '\.hg', '\.svn', '\.bzr', 'node_modules']

" turn off backup files
set nobackup
set nowb
set noswapfile

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
autocmd FileType javascript :set fmr=/*,*/ fdm=marker fdc=1
autocmd FileType cpp :set fmr=/**,*/ fdm=marker fdc=1
autocmd FileType c :set fmr=/**,*/ fdm=marker fdc=1

" complete options (disable preview scratch window)
set completeopt=menu,menuone,longest

" limit complete popup height
set pumheight=15

" auto read when file changed elsewhere
set autoread

" ignore compiled files
set wildignore=*.o,*~,*.hi,*.zip,*.so

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

let g:SuperTabDefaultCompletionType = "context"

let g:ctrlp_custom_ignore = '\v[\/]((\.(git|hg|svn))|node_modules)$'
let g:ctrlp_working_path_mode = 0

" airline configuration
let g:airline_enable_syntastic=1
let g:airline_powerline_fonts = 1

" if it's in an xterm window, use 256 colors
if $TERM == "xterm"
    set t_Co=256
endif

" start neocomplete at startup
if has("lua")
    let g:neocomplete#enable_at_startup = 1
else
    let g:neocomplete#enable_at_startup = 0
endif

"neco ghc
let g:necoghc_enable_detailed_browse = 1

" ---------------------------------------
"  Key mappings
" ---------------------------------------
let g:ctrlp_map = '<c-t>'
" Making it so ; works like : for commands. Saves typing and eliminates :W style typos due to lazy holding shift.
nnoremap ; :
"" bind F2 to toggle
nnoremap <F2> :call NumberToggle()<cr>
" mapping for Gundo
nnoremap <F5> :GundoToggle<CR>
" remove trailing whitespace with F7
nmap <F7> :%s/\s\+$//e<cr>
" tagbar toggle with F8
nmap <F8> :TagbarToggle<CR>
" command mapping from nt to NERDTree
nmap <silent> <leader>nt :NERDTreeToggle<CR>
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
" fast saving with \w
nmap <leader>w :w!<cr>

" Map space to search and c-space to backwards search
map <space> /
map <c-space> ?

" use \cd to change directory of the current file
map <leader>cd :cd %:p:h<cr>:pwd<cr>

" toggle paste mode
map <leader>pp :setlocal paste!<cr>
" ---------------------------------------
