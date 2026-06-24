
#ifndef SOFTOMNILIB_TOKEN_SET_HH
#define SOFTOMNILIB_TOKEN_SET_HH
#include <map>
#include <memory>
#include <string>
#include <vector>

#include <Parsing/Commons/Lexing/Tokens/token.hh>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::TokenSets
{
    template <typename TFrontend, std::derived_from<Token<TFrontend, > TToken>
    class TokenSet
    {
        TokenSet* parent_;

        std::string name_;

        std::string display_name_;

        std::map<std::string, std::reference_wrapper<TToken>> tokens_;

        std::vector<TokenSet> subsets_;

    public:
        explicit TokenSet(std::string name);

        explicit TokenSet(std::string name, TokenSet& parent);

        [[nodiscard]] std::string name() const;

        [[nodiscard]] std::string display_name() const;

        void set_display_name(std::string new_name);

        [[nodiscard]] std::size_t number_of_tokens() const;

        [[nodiscard]] std::size_t number_of_immediate_subsets() const;

        [[nodiscard]] std::size_t number_of_subsets_transitively() const;

        [[nodiscard]] bool contains_token(std::string token) const;

        [[nodiscard]] bool contains_subset(const TokenSet& token_set) const;

        void add_token(std::unique_ptr<TToken> token);

        void add_subset(std::unique_ptr<TokenSet> token_set);

        void remove_token(std::unique_ptr<TToken> token);

        void remove_subset(std::unique_ptr<TokenSet> token_set);

        void make_subset_out_of(std::string elements);

        void make_subset_out_of(std::iter elements);
    };
}

#endif //SOFTOMNILIB_TOKEN_SET_HH
