
#ifndef SOFTOMNILIB_ROOT_NODE_HH
#define SOFTOMNILIB_ROOT_NODE_HH

#include <Include/Parsing/Commons/AST/Base/Constrained/Internal/internal_node.hh>

namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Root
{
    template <typename TTreeGrouping, typename TFrontend, typename TCharType>
    class RootNode: private Internal::InternalNode<RootNode<TTreeGrouping, TFrontend, TCharType>, TTreeGrouping, TFrontend, TCharType>
    {
        using TBaseNode = Internal::InternalNode<RootNode, TTreeGrouping, TFrontend, TCharType>::TBaseNode;

    protected:
        explicit RootNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer);

        explicit RootNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            std::vector<TBaseNode> children);

        explicit RootNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            const Internal::iterable<TBaseNode> auto& children);

        template <typename ...Args> requires (std::convertible_to<Args, TBaseNode> && ...)
        explicit RootNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer, Args&&... children);

    public:
        InterconnectNode
    };
}

#endif //SOFTOMNILIB_ROOT_NODE_HH
