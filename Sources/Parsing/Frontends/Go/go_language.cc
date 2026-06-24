
#include <Parsing/Frontends/Go/go_language.hh>

namespace SoftOmni::Parsing::Frontends::Go
{
    GoLanguage::GoLanguage() : Language("go", "Go") {}

    const GoLanguage &GoLanguage::GetInstance()
    {
        const Language & language = get("go").value().get();
        return static_cast<const GoLanguage &>(language);
    }
} // namespace SoftOmni::Parsing::Frontends::Go
