
#include <Parsing/Frontends/JavaProperties/java_properties_language.hh>

namespace SoftOmni::Parsing::Frontends::JavaProperties
{
    JavaPropertiesLanguage::JavaPropertiesLanguage() : Language("java_properties", "Java Properties") {}

    const JavaPropertiesLanguage &JavaPropertiesLanguage::GetInstance()
    {
        const Language &language = get("java_properties").value().get();
        return static_cast<const JavaPropertiesLanguage &>(language);
    }
} // namespace SoftOmni::Parsing::Frontends::JavaProperties
