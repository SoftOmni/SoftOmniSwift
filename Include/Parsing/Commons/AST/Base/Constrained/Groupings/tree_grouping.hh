
#ifndef SOFTOMNILIB_TREE_GROUPING_HH
#define SOFTOMNILIB_TREE_GROUPING_HH

#include <Parsing/Commons/Kernels/language_frontend.hh>

namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Groupings
{
    template <typename TGrouping>
    concept ValidNodeBundle = requires
    {
        typename TGrouping::Base;
        typename TGrouping::Internal;
        typename TGrouping::Leaf;
        typename TGrouping::Root;
    }
    &&
        std::is_base_of_v<typename TGrouping::Internal, typename TGrouping::Root>
    &&
        std::is_base_of_v<typename TGrouping::Base, typename TGrouping::Internal>
    &&
        std::is_base_of_v<typename TGrouping::Base, typename TGrouping::Leaf>;


    template <typename TBase, typename TInternal, typename TLeaf, typename TRoot>
    struct NodeGrouping
    {
        using Base = TBase;
        using Internal = TInternal;
        using Leaf = TLeaf;
        using Root = TRoot;

        static_assert(std::is_base_of_v<TBase, TInternal>,
            "internal node type must inherit from base node type");

        static_assert(std::is_base_of_v<TBase, TLeaf>,
            "leaf node type must inherit from base node type");

        static_assert(std::is_base_of_v<TInternal, TRoot>,
            "root node type must inherit from internal node type");
    };
}

#endif //SOFTOMNILIB_TREE_GROUPING_HH
