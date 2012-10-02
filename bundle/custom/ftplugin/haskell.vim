autocmd BufWritePost *.hs GhcModCheckAndLintAsync
let g:neocomplcache_enable_at_startup = 1

map <leader>gt :GhcModType<cr>
map <leader>gi :GhcModInfo<cr>
map <leader>ge :GhcModExpand<cr>
map <leader>gc :GhcModCheck<cr>
map <leader>gl :GhcModLint<cr>

"if has("autocmd")
  "autocmd Filetype haskell setlocal omnifunc=necoghc#omnifunc 
"endif
