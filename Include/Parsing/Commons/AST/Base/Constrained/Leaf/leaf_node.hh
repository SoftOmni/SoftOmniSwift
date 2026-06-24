
#ifndef SOFTOMNILIB_LEAF_NODE_HH
#define SOFTOMNILIB_LEAF_NODE_HH

#include <Parsing/Commons/AST/Base/Constrained/Base/node.hh>

namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Leaf
{
    template <typename TTreeGrouping, typename TFrontend, typename TCharType>
    class LeafNode: public Base::Node<LeafNode<TTreeGrouping, TFrontend, TCharType>, TTreeGrouping, TFrontend, TCharType>
    {
        explicit LeafNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer);

        explicit LeafNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer, TTreeGrouping::Internal& parent, ParentOffsets::ParentOffset parent_offset);
    };
}

#endif //SOFTOMNILIB_LEAF_NODE_HH
