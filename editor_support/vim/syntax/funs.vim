" funs syntax highlighting

if exists("b:current_syntax")
  finish
endif

syntax clear

" TODO
syn keyword funsTodo TODO FIXME XXX NOTE
hi def link funsTodo Todo

" Keywords
syn keyword funsControl
    \ if
    \ then
    \ else
    \ match
hi def link funsControl Keyword

" Types
syn keyword funsType
    \ int
    \ float
    \ str
    \ bool
    \ unit
    \ data
hi def link funsType Type

" Strings
syn match funsString /".*"/
hi def link funsString String

" Delimiters
syn match funsDelimiter /[\(\)\[\]\{\},;:_]/
hi def link funsDelimiter Delimiter

" Operators
syn match funsOperator /[-+*\/=<>|]/
hi def link funsOperator Operator

" Constants
syn keyword funsConstant true false
hi def link funsConstant Constant

" Comments
syn match funsComment "#.*$" contains=smallerbasicTodo,@Spell
hi def link funsComment Comment

" Variables
syn match funsVariable /\v\w+/
hi def link funsVariable Identifier

" Numbers
syn match funsNumber "\v<\d+>"
syn match funsNumber "\v<\d+\.\d+>"
syn match funsNumber "\v<\d*\.?\d+([Ee]-?)?\d+>"
syn match funsNumber "\v<0x\x+([Pp]-?)?\x+>"
syn match funsNumber "\v<0b[01]+>"
syn match funsNumber "\v<0o\o+>"
" hi def link funsNumber Number
hi def link funsNumber PreProc

let b:current_syntax = "funs"

