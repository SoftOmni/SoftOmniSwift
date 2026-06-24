
#ifndef SOFTOMNILIB_INCREMENTAL_LEXER_HH
#define SOFTOMNILIB_INCREMENTAL_LEXER_HH

#include <Include/Parsing/Commons/Lexing/lexer.hh>

namespace SoftOmni::Parsing::Commons::Lexing
{
    template <typename TToken>
    class IncrementalLexer: virtual Lexer<TToken>
    {
    protected:
        explicit IncrementalLexer();

    public:
        virtual void advance() = 0;

        virtual ~IncrementalLexer();
    };
}

#endif //SOFTOMNILIB_INCREMENTAL_LEXER_HH
