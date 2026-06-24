

#ifndef SOFTOMNILIB_GO_LANGUAGE_HH
#define SOFTOMNILIB_GO_LANGUAGE_HH

#include <Parsing/Commons/Kernels/language.hh>

namespace SoftOmni::Parsing::Frontends::Go
{
    class GoLanguage final : public Commons::Kernels::Language
    {
        explicit GoLanguage();

    public:
        GoLanguage(GoLanguage &other) = delete;

        void operator=(const GoLanguage &) = delete;

        [[nodiscard]] static const GoLanguage &GetInstance();
    };
} // namespace SoftOmni::Parsing::Frontends::Go

#endif // SOFTOMNILIB_GO_LANGUAGE_HH
