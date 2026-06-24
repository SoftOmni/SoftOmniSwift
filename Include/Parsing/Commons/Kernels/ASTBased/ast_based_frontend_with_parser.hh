

#ifndef SOFTOMNILIB_AST_BACKED_FRONTEND_WITH_PARSER_HH
#define SOFTOMNILIB_AST_BACKED_FRONTEND_WITH_PARSER_HH

#include <Parsing/Commons/Kernels/ASTBased/ast_based_frontend.hh>

namespace SoftOmni::Parsing::Commons::Kernels::ASTBased
{
    template <typename TDerivedASTFrontend, typename TTreeGroup, typename TCharType = char>
    class ASTBasedFrontendWithParser : ASTBasedFrontend<TDerivedASTFrontend, TTreeGroup, TCharType>
    {
        static_assert(std::is_base_of_v<ASTBasedFrontendWithParser, TDerivedASTFrontend>, "TDerivedASTFrontend must be derived from ASTBasedFrontendWithFrontend");

        Parsing::Parser<TDerivedASTFrontend> parser_;

    protected:

        explicit ASTBasedFrontendWithParser(const std::string &name, const std::string &language, Parsing::Parser<TDerivedASTFrontend> parser);

        explicit ASTBasedFrontendWithParser(const std::string &name, Language &language, Parsing::Parser<TDerivedASTFrontend> parser);

        explicit ASTBasedFrontendWithParser(std::string name, std::string display_name, const std::string &language, Parsing::Parser<TDerivedASTFrontend> parser);

        explicit ASTBasedFrontendWithParser(std::string name, std::string display_name, Language& language, Parsing::Parser<TDerivedASTFrontend> parser);

    public:
        [[nodiscard]] const Parsing::Parser<TDerivedASTFrontend>& parser() const;

        void add_buffer(Buffers::Buffer<TCharType> buffer);
    };
}

#endif // SOFTOMNILIB_AST_BACKED_FRONTEND_WITH_PARSER_HH
