
#ifndef SOFTOMNILIB_RANDOM_ACCESS_LEXER_HH
#define SOFTOMNILIB_RANDOM_ACCESS_LEXER_HH

#include <Include/Parsing/Commons/Lexing/lexer.hh>

namespace SoftOmni::Parsing::Commons::Lexing
{
    template <typename TToken>
    class RandomAccessLexer : virtual Lexer<TToken>
    {
        using TokenReturnType = Lexer<TToken>::TokenReturnType;

    protected:
        explicit RandomAccessLexer();

    public:
        [[nodiscard]] virtual std::optional<TokenReturnType> operator[](std::size_t index) const = 0;
    };
}

#endif //SOFTOMNILIB_RANDOM_ACCESS_LEXER_HH
