

#ifndef SOFTOMNILIB_SCOPED_MAP_RESOLVER_HH
#define SOFTOMNILIB_SCOPED_MAP_RESOLVER_HH

#include <Parsing/Commons/Kernels/Features/Analyzable/Semantics/resolver_namespace_with_inner_scopes.hh>
#include <stack>

namespace SoftOmni::Parsing::Commons::Kernels::Features::Analyzable::Semantics
{
    template <typename TNodeType>
    class ScopedMapResolver
    {
        std::stack<ResolverNamespace<TNodeType>> maps_;

    public:
        explicit ScopedMapResolver();

        [[nodiscard]] bool contains_in_scope(std::string value) const;

        [[nodiscard]] bool contains_in_scope(const Symbol& value) const;

        void enter_scope();

        void leave_scope();
    };
}

#endif // SOFTOMNILIB_SCOPED_MAP_RESOLVER_HH
