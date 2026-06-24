
#ifndef SOFTOMNILIB_LEXER_HH
#define SOFTOMNILIB_LEXER_HH
#include <optional>
#include <type_traits>

namespace SoftOmni::Parsing::Commons::Lexing
{
    enum class LexerState
    {
        NOT_LEXED,
        PARTIALLY_LEXED,
        LEXING,
        INCREMENTALLY_LEXING,
        OUT_OF_DATE,
        OUT_OF_DATE_PARTIALLY_LEXED,
        OUT_OF_DATE_LOCALLY,
        OUT_OF_DATE_LOCALLY_PARTIALLY_LEXED,
        LEXED
    };

    template <typename TToken>
    class Lexer
    {
    protected:
        std::size_t number_of_lexed_tokens_;

        LexerState parser_state_;

        using TokenReturnType = std::conditional_t<sizeof(TToken) < 2 * sizeof(std::size_t), TToken, const TToken &>;

        explicit Lexer();

    public:
        [[nodiscard]] virtual LexerState lexer_state() const = 0;

        [[nodiscard]] bool is_lexed() const;

        [[nodiscard]] bool is_lexing() const;

        [[nodiscard]] virtual std::optional<TokenReturnType> last_lexed_token() = 0;

        [[nodiscard]] virtual std::optional<TokenReturnType> before_last_lexed_token() = 0;

        [[nodiscard]] std::size_t number_of_lexed_tokens() const;

        virtual void lex() = 0;

        virtual void relex() = 0;

        virtual ~Lexer();
    };
} // namespace SoftOmni::Parsing::Commons::Lexing

#endif // SOFTOMNILIB_LEXER_HH
