# DOD parser / language server in rust
> **NOTE:** this is an educational project, I wanted to learn about compilers, parsers, lsps and rust, so I built this.

## tokenizer

The tokenizer is the first part of the compilation process and it is very simple.  
Given an iterator of characters, it is supposed to group them by token (e.g. 'if', 'let', 'for', '{', etc...).  

## Parser

The next step involves taking the tokens and try to build an AST ([A]bstract [S]yntax [T]ree), which will try to make sense of those tokens.  
If it fails to do so, then there must be some kind of syntax issue.  

## LSP ([L]anguage [S]erver [P]rotocol)

The language server protocol is a protocol that allows for the communication between editors/IDEs and the (Language Servers), kinda like an API.  
They could communicate via TCP or STDIN-STDOUT and I choose the latter for this project.  

To learn more check the [lsp specifications](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification)



