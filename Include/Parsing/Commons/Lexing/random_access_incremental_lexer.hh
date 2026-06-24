
#ifndef SOFTOMNILIB_RANDOM_ACCESS_INCREMENTAL_LEXER_HH
#define SOFTOMNILIB_RANDOM_ACCESS_INCREMENTAL_LEXER_HH

#include <Include/Parsing/Commons/Lexing/random_access_lexer.hh>
#include <Include/Parsing/Commons/Lexing/incremental_lexer.hh>

namespace SoftOmni::Parsing::Commons::Lexing
{
    template <typename TToken>
    class RandomAccessIncrementalLexer: virtual IncrementalLexer<TToken>, virtual RandomAccessLexer<TToken>
    {
    protected:
        explicit RandomAccessIncrementalLexer();
    };
}

#endif //SOFTOMNILIB_RANDOM_ACCESS_INCREMENTAL_LEXER_HH
