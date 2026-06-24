

#ifndef SOFTOMNILIB_GO_FRONTEND_HH
#define SOFTOMNILIB_GO_FRONTEND_HH

#include <Parsing/Commons/Kernels/language_frontend.hh>
#include <Parsing/Frontends/Go/go_language.hh>

namespace SoftOmni::Parsing::Frontends::Go
{
    class GoFrontend : public Commons::Kernels::LanguageFrontend
    {
    protected:
        explicit GoFrontend(const std::string& name, const std::string& language);

        explicit GoFrontend(const std::string& name, const GoLanguage& language);

        GoFrontend(std::string name, std::string display_name, const std::string& language);

        GoFrontend(std::string name, std::string display_name, const GoLanguage& language);
    };
} // namespace SoftOmni::Parsing::Frontends::Go

#endif // SOFTOMNILIB_GO_FRONTEND_HH
