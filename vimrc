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

if has("autocmd")
  filetype plugin indent on
endif

" command mapping from nt to NERDTree
nmap <silent> <leader>nt :NERDTreeToggle<CR>
let NERDTreeIgnore=['\.pyc', '\~$', '\.swo$', '\.swp$', '\.git', '\.hg', '\.svn', '\.bzr']

" Easier moving in tabs and windows
map <C-J> <C-W>j
map <C-K> <C-W>k
map <C-L> <C-W>l
map <C-H> <C-W>h
map <C-K> <C-W>k

" Stupid shift key fixes
cmap W w 						
cmap WQ wq
cmap wQ wq
cmap Q q

" turn off backup files
set nobackup

" Making it so ; works like : for commands. Saves typing and eliminates :W style typos due to lazy holding shift.
nnoremap ; :

"OmniCppComplete
set omnifunc=syntaxcomplete#Complete
let OmniCpp_GlobalScopeSearch = 1
let OmniCpp_ShowAccess = 1
let OmniCpp_ShowPrototypeInAbbr = 1 " show function parameters
let OmniCpp_MayCompleteDot = 1 " autocomplete after .
let OmniCpp_MayCompleteArrow = 1 " autocomplete after ->
let OmniCpp_MayCompleteScope = 1 " autocomplete after ::
let OmniCpp_DefaultNamespaces = ["std", "_GLIBCXX_STD"]

" mapping for Gundo
nnoremap <F5> :GundoToggle<CR>

" Turn on brief-mode for JavaScript indenter
let g:SimpleJsIndenter_BriefMode = 1
