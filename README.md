Just a repository with the sources for Latex presentations

Since the compilation and cleaning process is 
basically two commands you can work without any Makefiles!

```bash
# Compiles the pdf file. Don't mind the bibliography warning, just press 'enter'
pdflatex -shell-escape main.tex
# Cleans all of the debug/compile trash
rm main.{toc,run.xml,.tex.bbl,tex.blg,log,nav,bcf,aux,out,snm,fdb_latexmk,bbl,blg}

# When you have to turn slides into lecture notes for yourself, copy the
# first 30 lines or so of configs from any lecture.tex file, and then just
# apply some of these commands to make it compile and look nice:
# :%s/\\\\/
# :%s/\\framesubtitle{\(.\+\)}/\\subsubsection{\1}/
# :%s/\\end{frame}
# :%s/fontsize=\\large/fontsize=\\normalsize/
# :%s/\\begin{frame}{\(.\+\)}/\\subsection{\1}/
# :%s/\\textcolor{.\+}{\(.\+\)}/\1/
# :%s/\\vspace.\+//
```

I've also included my `tex.snippets` file for the `vim-snippets`
plugin, greatly increases the speed of my workflow

Here is the minimal nvim config using Plugged to get you started:
```vim
" Insert this in your plugin section
" Latex plugin
Plug 'lervag/vimtex'

" (you probably need to have something like coc.vim set up)
" Setting up the snippet engine I mostly use for Latex
" They are super awesome, the list is here https://github.com/honza/vim-snippets/blob/master/UltiSnips/tex.snippets
Plug 'SirVer/ultisnips'
Plug 'honza/vim-snippets'

" And this after closing off your plugins, choose your own keys:
let g:UltiSnipsExpandTrigger="<C-d>"
let g:UltiSnipsJumpForwardTrigger="<C-f>"
let g:UltiSnipsJumpBackwardTrigger="<C-s>"
```
