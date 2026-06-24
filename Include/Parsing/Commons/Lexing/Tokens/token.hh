//
// Created by jaschamerle on 4/29/26.
//

#ifndef SOFTOMNILIB_TOKEN_HH
#define SOFTOMNILIB_TOKEN_HH
#include <string>

#include <Include/Parsing/Commons/Kernels/language_frontend.hh>
#include <Include/Parsing/Commons/Lexing/Tokens/TokenSets/frontend_token_set.hh>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens
{
    template <std::derived_from<Kernels::LanguageFrontend> TFrontend, typename DerivedTokenType>
    class Token
    {
        std::string name_;

        const TokenSets::FrontendTokenSet<TFrontend, DerivedTokenType>& main_token_set_;

    public:
        constexpr Token(std::string name, const TokenSets::FrontendTokenSet<TFrontend, DerivedTokenType>& token_set)
        : name_(std::move(name)), main_token_set_(token_set)
        {
            static_assert(std::derived_from<DerivedTokenType, Token>,
                "DerivedTokenType must derive from Token");
        }

        [[nodiscard]] std::string name() const;

        [[nodiscard]] TokenSets::FrontendTokenSet<TFrontend, DerivedTokenType>& token_set() const;
    };
}

#endif //SOFTOMNILIB_TOKEN_HH
