
#ifndef SOFTOMNILIB_RESOLVER_NAMESPACE_WITH_INNER_SCOPES_HH
#define SOFTOMNILIB_RESOLVER_NAMESPACE_WITH_INNER_SCOPES_HH

#include <Parsing/Commons/Kernels/Features/Analyzable/Semantics/resolver_namespace.hh>
#include <memory>
#include <set>

namespace SoftOmni::Parsing::Commons::Kernels::Features::Analyzable::Semantics
{
    template <typename TTargetNode>
    class ResolverNamespaceWithInnerScopes : public ResolverNamespace<TTargetNode>
    {
        std::set<ResolverNamespaceWithInnerScopes> inner_scopes_;

        std::unordered_map<Symbol, std::reference_wrapper<TTargetNode>> all_nodes_map_;

        std::unordered_set<std::reference_wrapper<TTargetNode>> all_nodes_;

    public:
        explicit ResolverNamespaceWithInnerScopes();

        explicit ResolverNamespaceWithInnerScopes(std::size_t initial_capacity);

        explicit ResolverNamespaceWithInnerScopes(std::vector<TTargetNode&> declarations);

        explicit ResolverNamespaceWithInnerScopes(std::vector<TTargetNode&> declarations, std::size_t initial_capacity);

        template <std::size_t size>
        explicit ResolverNamespaceWithInnerScopes(std::array<TTargetNode&, size> declarations);

        template <std::size_t size>
        explicit ResolverNamespaceWithInnerScopes(std::array<TTargetNode&, size> declarations, std::size_t initial_capacity);

        template <typename... Args> requires(std::convertible_to<Args, TTargetNode&> && ...)
        explicit ResolverNamespaceWithInnerScopes(Args&& ...args);

        template <typename... Args> requires(std::convertible_to<Args, TTargetNode&> && ...)
        explicit ResolverNamespaceWithInnerScopes(Args&& ...args, std::size_t initial_capacity);

        [[nodiscard]] bool contains_not_in_children(const std::string& name) const;

        [[nodiscard]] bool contains_not_in_children(const Symbol& symbol) const;

        [[nodiscard]] bool contains_not_in_children(const TTargetNode& declaration) const;

        std::set<ResolverNamespaceWithInnerScopes>::const_iterator inner_scopes_cbegin() const;

        std::set<ResolverNamespaceWithInnerScopes>::const_iterator inner_scopes_cend() const;

        std::set<ResolverNamespaceWithInnerScopes>::iterator inner_scopes_begin();

        std::set<ResolverNamespaceWithInnerScopes>::iterator inner_scopes_end();

        // TODO: Add depth-first search through all children
    };
} // namespace SoftOmni::Parsing::Commons::Kernels::Features::Analyzable::Semantics

#endif // SOFTOMNILIB_RESOLVER_NAMESPACE_WITH_INNER_SCOPES_HH
