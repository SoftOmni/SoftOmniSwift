
#ifndef SOFTOMNILIB_FRONTEND_TOKEN_SET_HH
#define SOFTOMNILIB_FRONTEND_TOKEN_SET_HH

#include <Include/Parsing/Commons/Lexing/Tokens/TokenSets/token_set.hh>
#include <Include/Parsing/Commons/Kernels/language_frontend.hh>
#include <unordered_map>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::TokenSets
{
    template <std::derived_from<Kernels::LanguageFrontend> TFrontend, std::derived_from<Token> TToken>
    class FrontendTokenSet : public TokenSet<TToken>
    {
        const TFrontend* frontend_;

        static std::unordered_map<TFrontend, FrontendTokenSet> frontend_token_sets_;

    public:
        FrontendTokenSet(const TFrontend& frontend, std::string name);

        [[nodiscard]] const TFrontend& frontend() const;

        ~FrontendTokenSet();

        friend Kernels::LanguageFrontend;

    private:
        static void discard_token_set_of_frontend(const TFrontend& frontend);
    };
}

#endif //SOFTOMNILIB_FRONTEND_TOKEN_SET_HH
