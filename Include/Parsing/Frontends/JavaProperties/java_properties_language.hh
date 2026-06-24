

#ifndef SOFTOMNILIB_JAVA_PROPERTIES_LANGUAGE_HH
#define SOFTOMNILIB_JAVA_PROPERTIES_LANGUAGE_HH

#include <Parsing/Commons/Kernels/language.hh>

namespace SoftOmni::Parsing::Frontends::JavaProperties
{
    class JavaPropertiesLanguage : public Commons::Kernels::Language
    {
        explicit JavaPropertiesLanguage();

    public:
        JavaPropertiesLanguage(JavaPropertiesLanguage &other) = delete;

        void operator=(const JavaPropertiesLanguage &) = delete;

        [[nodiscard]] static const JavaPropertiesLanguage &GetInstance();
    };
} // namespace SoftOmni::Parsing::Frontends::JavaProperties

#endif // SOFTOMNILIB_JAVA_PROPERTIES_LANGUAGE_HH
