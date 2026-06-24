
#ifndef SOFTOMNILIB_NODE_HH
#define SOFTOMNILIB_NODE_HH

#include <Parsing/Commons/Kernels/language_frontend.hh>

#include <Parsing/Commons/AST/Base/ParentOffsets/parent_offset.hh>
#include <Parsing/Commons/Buffers/buffer.hh>


namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Internal
{
    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename TCharType>
    class InternalNode;
}

namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Base
{
    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
    requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    class Node
    {
    protected:
        using InternalType = TTreeGrouping::Internal;

    public:
        const TFrontend* frontend_;

        Buffers::Buffer<CharType>* buffer_;

        InternalType* parent_;

        std::optional<ParentOffsets::ParentOffset> offset_;

    protected:
        explicit Node(TFrontend& frontend, Buffers::Buffer<CharType>& buffer);

        explicit Node(TFrontend& frontend, Buffers::Buffer<CharType>& buffer, InternalType& parent,
                      ParentOffsets::ParentOffset parent_offset);

        Buffers::Buffer<CharType>& editable_buffer();

    public:
        [[nodiscard]] const Kernels::LanguageFrontend& frontend() const;

        [[nodiscard]] const Kernels::Language& language() const;

        [[nodiscard]] const Buffers::Buffer<CharType>& buffer() const;

        [[nodiscard]] std::optional<std::reference_wrapper<InternalType>> parent() const;

        [[nodiscard]] bool has_parent() const;

        [[nodiscard]] bool is_root_of_tree() const;

        [[nodiscard]] std::optional<std::size_t> index_in_parent() const;

        [[nodiscard]] std::optional<std::size_t> text_index_in_parent() const;

        void detach_from_parent();

        void attach_to_parent(InternalType& new_parent, ParentOffsets::ParentOffset new_parent_offset);

    protected:
        void unsafe_forcibly_attach_to_parent(InternalType& new_parent, ParentOffsets::ParentOffset new_parent_offset);
    };
} // SoftOmni::Parsing::Commons::AST::Base::Constrained::Base

#include <Parsing/Commons/AST/Base/Constrained/Base/templated_node.hxx>

#endif //SOFTOMNILIB_NODE_HH
