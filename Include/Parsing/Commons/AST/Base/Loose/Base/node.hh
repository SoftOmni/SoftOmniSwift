
#ifndef SOFTOMNILIB_NODE_HH
#define SOFTOMNILIB_NODE_HH

#include <Parsing/Commons/Kernels/language_frontend.hh>

namespace SoftOmni::Parsing::Commons::AST::Base::Loose::Base
{
    class Node
    {
        const Kernels::LanguageFrontend* frontend_;

        Buffers::EditableBuffer* buffer_;

        Loose::Internal::InternalNode* parent_;

    protected:
        explicit Node(const Kernels::LanguageFrontend& frontend, Buffers::EditableBuffer& buffer,
            Loose::Internal::InternalNode parent);

        [[nodiscard]] const Kernels::LanguageFrontend& frontend() const;

        [[nodiscard]] const Kernels::Language& language() const;

        [[nodiscard]] const Buffers::Buffer& buffer() const;

        [[nodiscard]] std::optional<std::reference_wrapper<Node>> parent() const;

        [[nodiscard]] bool has_parent() const;

        [[nodiscard]] bool is_root_of_tree() const;
    };
} // SoftOmni::Parsing::Commons::AST::Base::Loose::Base

#endif //SOFTOMNILIB_NODE_HH
