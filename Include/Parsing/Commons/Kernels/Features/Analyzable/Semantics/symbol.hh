

#ifndef SOFTOMNILIB_SYMBOL_HH
#define SOFTOMNILIB_SYMBOL_HH
#include <string>
#include <unordered_map>
#include <unordered_set>


namespace SoftOmni::Parsing::Commons::Kernels::Features::Analyzable::Semantics
{
    class Symbol final
    {
        static std::unordered_map<std::string, Symbol*> symbols_;

        std::string value_;

        explicit Symbol(std::string value);

    public:
        static Symbol& for_value(const std::string& value);

        Symbol(const Symbol&) = delete;
        Symbol& operator=(const Symbol&) = delete;

        // ReSharper disable once CppNonExplicitConversionOperator
        operator const std::string&() const;

        // ReSharper disable once CppNonExplicitConversionOperator
        operator std::string&();

        ~Symbol();
    };
}

#endif // SOFTOMNILIB_SYMBOL_HH
