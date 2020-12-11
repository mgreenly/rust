# Samples

I like to make it convenient to build/run single file experiments when learning a lanuage

I use per directory .vimrc files for this, mapping <leader>r to build and run the file

   map <leader>r :execute '!clear && rustc -o %:r % && ./%:r'<CR>
