

#ifndef SOFTOMNILIB_SEMANTICALLY_ENABLED_AST_BASED_FRONTEND_HH
#define SOFTOMNILIB_SEMANTICALLY_ENABLED_AST_BASED_FRONTEND_HH

#include <Parsing/Commons/Kernels/ASTBased/ast_based_frontend.hh>

namespace SoftOmni::Parsing::Commons::Kernels::ASTBased
{
    template <typename TDerivedASTFrontend, typename TTreeGroup, typename TCharType = char>
    class SemanticallyEnabledASTBasedFrontend : ASTBasedFrontend<TDerivedASTFrontend, TTreeGroup, TCharType>
    {
        static_assert(std::is_base_of_v<SemanticallyEnabledASTBasedFrontend, TDerivedASTFrontend>,
                      "TDerivedASTFrontend must be derived from SemanticallyEnabledASTBasedFrontend");

        Parsing::Parser<TDerivedASTFrontend> parser_;

    protected:
        explicit SemanticallyEnabledASTBasedFrontend(const std::string &name, const std::string &language);

        explicit SemanticallyEnabledASTBasedFrontend(const std::string &name, Language &language);

        explicit SemanticallyEnabledASTBasedFrontend(std::string name, std::string display_name, const std::string &language);

        explicit SemanticallyEnabledASTBasedFrontend(std::string name, std::string display_name, Language &language);
    };
} // namespace SoftOmni::Parsing::Commons::Kernels::ASTBased

#endif // SOFTOMNILIB_SEMANTICALLY_ENABLED_AST_BASED_FRONTEND_HH
