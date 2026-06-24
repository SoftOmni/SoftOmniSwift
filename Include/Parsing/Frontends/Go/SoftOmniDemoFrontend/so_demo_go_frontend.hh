

#ifndef SOFTOMNILIB_SO_DEMO_GO_FRONTEND_HH
#define SOFTOMNILIB_SO_DEMO_GO_FRONTEND_HH

#include <Parsing/Frontends/Go/go_frontend.hh>
#include <Parsing/Commons/Kernels/ASTBased/semantically_enabled_ast_based_frontend_with_parser.hh>

namespace SoftOmni::Parsing::Frontends::Go::SoftOmniDemoFrontend
{
    class SoftOmniDemoGoFrontend final : public GoFrontend, Commons::Kernels::ASTBased::SemanticallyEnabledASTBasedFrontend<>
    {
        explicit SoftOmniDemoGoFrontend(const std::string& name, const std::string& language);

        explicit SoftOmniDemoGoFrontend(const std::string& name, const GoLanguage& language);

        SoftOmniDemoGoFrontend(std::string name, std::string display_name, const std::string& language);

        SoftOmniDemoGoFrontend(std::string name, std::string display_name, const GoLanguage& language);
    };
} // namespace SoftOmni::Parsing::Frontends::Go

#endif // SOFTOMNILIB_SO_DEMO_GO_FRONTEND_HH
