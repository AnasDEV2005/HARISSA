
syntax keyword harissaKeyword fn use pub const while loop object enum return run if else elif ERROR_RULES
syntax keyword harissaType int string list tup couple bool crate
syntax keyword harissaBool true false
syntax match harissaComment "//.*$"
syntax region harissaCommentBlock start="/\*" end="*/"
syntax region harissaString start=/"/ end=/"/
syntax region harissaString start=/'/ end=/'/
syntax match harissaIdent "\<[A-Za-z_][A-Za-z0-9_]*\>"
syntax match harissaFunction "\<[A-Za-z_][A-Za-z0-9_]*\>\s*("me=e-1
syntax match harissaIn "->"
syntax match harissaSymbol "||" 
syntax match harissaSym "&&"
syntax match harissaDD "::"
syntax match harissaSy "=="
syntax match harissaS "=>"
syntax match harissaSymb "=<"
syntax match harissaParam /\<[A-Z][A-Za-z0-9_]*:\>/
syntax match harissaObject "\<[A-Z][A-Za-z0-9_]*\>"
syntax match harissaNum "\<[0-9_]*\>"



hi def link harissaKeyword Keyword
hi def link harissaType Type
hi def link harissaComment Comment
hi def link harissaCommentBlock Comment
hi def link harissaString String
highlight link harissaParam Identifier
highlight link harissaObject Constant
highlight link harissaFunction Identifier
highlight link harissaIdent Function
highlight link harissaIn Identifier
highlight link harissaNum Constant
highlight link harissaS Identifier 
highlight link harissaSy Identifier 
highlight link harissaSymbol Identifier 
highlight link harissaSym Identifier 
highlight link harissaSymb Identifier 
highlight link harissaBool Constant
highlight link harissaDD SpecialChar
