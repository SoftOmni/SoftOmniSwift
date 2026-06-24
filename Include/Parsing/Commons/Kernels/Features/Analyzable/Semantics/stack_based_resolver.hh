

#ifndef SOFTOMNILIB_STACK_BASED_RESOLVER_HH
#define SOFTOMNILIB_STACK_BASED_RESOLVER_HH

#include <Parsing/Commons/Kernels/Features/Analyzable/Semantics/resolver_namespace_with_inner_scopes.hh>
#include <stack>

namespace SoftOmni::Parsing::Commons::Kernels::Features::Analyzable::Semantics
{
    template <typename TNodeType>
    class StackBasedResolver
    {
        std::stack<TNodeType> nodes_;

    public:
        explicit StackBasedResolver();

        TNodeType& top_node() const;

        void push_node(TNodeType node);

        void pop_node();
    };
}

#endif // SOFTOMNILIB_STACK_BASED_RESOLVER_HH
