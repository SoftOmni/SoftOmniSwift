

#ifndef SOFTOMNILIB_RESOLVER_NAMESPACE_HH
#define SOFTOMNILIB_RESOLVER_NAMESPACE_HH
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include <Parsing/Commons/Kernels/Features/Analyzable/Semantics/symbol.hh>

namespace SoftOmni::Parsing::Commons::Kernels::Features::Analyzable::Semantics
{
    template <typename TTargetNode>
    class ResolverNamespace
    {
        std::unordered_map<Symbol, std::reference_wrapper<TTargetNode>> mapped_declarations_;

        std::unordered_set<std::reference_wrapper<TTargetNode>> declarations_;

    public:
        explicit ResolverNamespace();

        explicit ResolverNamespace(std::size_t initial_capacity);

        explicit ResolverNamespace(std::vector<TTargetNode&> declarations);

        explicit ResolverNamespace(std::vector<TTargetNode&> declarations, std::size_t initial_capacity);

        template <std::size_t size>
        explicit ResolverNamespace(std::array<TTargetNode&, size> declarations);

        template <std::size_t size>
        explicit ResolverNamespace(std::array<TTargetNode&, size> declarations, std::size_t initial_capacity);

        template <typename... Args> requires(std::convertible_to<Args, TTargetNode&> && ...)
        explicit ResolverNamespace(Args&& ...args);

        template <typename... Args> requires(std::convertible_to<Args, TTargetNode&> && ...)
        explicit ResolverNamespace(Args&& ...args, std::size_t initial_capacity);

        [[nodiscard]] std::size_t number_of_declarations() const;

        [[nodiscard]] bool contains(const std::string& name) const;

        [[nodiscard]] bool contains(const Symbol& symbol) const;

        [[nodiscard]] bool contains(const TTargetNode& declaration) const;

        void add_declaration(TTargetNode& declaration);

        void add_declaration(std::string declaration);

        void add_declarations(std::vector<TTargetNode&> declarations);

        template <std::size_t size>
        void add_declarations(std::array<TTargetNode&, size> declarations);

        template <typename... Args> requires(std::convertible_to<Args, TTargetNode&> && ...)
        void add_declarations(Args&& ...args);

        bool remove_declaration(TTargetNode& declaration);

        bool remove_declaration(std::string declaration);

        bool remove_declaration(Symbol declaration);

        std::unordered_map<Symbol, std::reference_wrapper<TTargetNode>>::const_iterator cbegin() const;

        std::unordered_map<Symbol, std::reference_wrapper<TTargetNode>>::const_iterator cend() const;

        std::unordered_map<Symbol, std::reference_wrapper<TTargetNode>>::iterator begin();

        std::unordered_map<Symbol, std::reference_wrapper<TTargetNode>>::iterator end();
    };
}

#endif // SOFTOMNILIB_RESOLVER_NAMESPACE_HH
