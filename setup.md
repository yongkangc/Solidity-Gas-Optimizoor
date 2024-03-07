# Setup

Running the program: `cargo run`

## Libraries

Logos:

- when using logos, you don't manually manipulate the lexer's current token, but rather define token patterns and let logos do the work of lexing and tokenization for you. If you need to perform additional processing on tokens, you do that at a higher level, usually in a loop that retrieves each token from the lexer.

## Design

### Lexer

- The tokenizer, also known as a lexical analyzer or lexer, is the component that takes raw input text and converts it into a stream of tokens. Tokens are the basic building blocks of a language's syntax, such as keywords, identifiers, literals, operators, and punctuation symbols.
