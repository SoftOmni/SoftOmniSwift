
#include <Parsing/Commons/Kernels/Features/Analyzable/Semantics/symbol.hh>
#include <utility>

namespace SoftOmni::Parsing::Commons::Kernels::Features::Analyzable::Semantics
{

    Symbol::Symbol(std::string value) : value_(std::move(value)) {}

    Symbol &Symbol::for_value(const std::string &value)
    {
        if (symbols_.contains(value))
        {
            return *symbols_.at(value);
        }


        Symbol* symbol = new Symbol(value);
        symbols_[value] = symbol;
        return *symbol;
    }

    Symbol::operator const std::string &() const
    {
        return value_;
    }

    Symbol::operator std::string&()
    {
        return value_;
    }

    Symbol::~Symbol()
    {
        symbols_.erase(value_);
    }
} // namespace SoftOmni::Parsing::Commons::Kernels::Features::Semantics
