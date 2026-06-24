#pragma once

namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Base
{
    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    Node<TDerived, TTreeGrouping, TFrontend, CharType>::Node(TFrontend &frontend, Buffers::Buffer<CharType> &buffer) :
        frontend_(frontend), buffer_(buffer), parent_()
    {
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    Node<TDerived, TTreeGrouping, TFrontend, CharType>::Node(TFrontend &frontend, Buffers::Buffer<CharType> &buffer,
                                                             InternalType &parent,
                                                             ParentOffsets::ParentOffset parent_offset) :
        frontend_(frontend), buffer_(buffer), parent_(parent), offset_(parent_offset)
    {
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    Buffers::Buffer<CharType> &Node<TDerived, TTreeGrouping, TFrontend, CharType>::editable_buffer()
    {
        return buffer_;
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    const Kernels::LanguageFrontend &Node<TDerived, TTreeGrouping, TFrontend, CharType>::frontend() const
    {
        return frontend_;
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    const Kernels::Language &Node<TDerived, TTreeGrouping, TFrontend, CharType>::language() const
    {
        return frontend_->language();
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    const Buffers::Buffer<CharType> &Node<TDerived, TTreeGrouping, TFrontend, CharType>::buffer() const
    {
        return buffer_;
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    std::optional<std::reference_wrapper<typename Node<TDerived, TTreeGrouping, TFrontend, CharType>::InternalType>>
    Node<TDerived, TTreeGrouping, TFrontend, CharType>::parent() const
    {
        if (parent_ == nullptr)
        {
            return std::nullopt;
        }

        return std::ref(parent_);
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    bool Node<TDerived, TTreeGrouping, TFrontend, CharType>::has_parent() const
    {
        return parent_ != nullptr;
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    bool Node<TDerived, TTreeGrouping, TFrontend, CharType>::is_root_of_tree() const
    {
        return parent_ == nullptr;
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    std::optional<std::size_t> Node<TDerived, TTreeGrouping, TFrontend, CharType>::index_in_parent() const
    {
        if (parent_ == nullptr)
        {
            return std::nullopt;
        }

        return offset_->index_in_parent();
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    std::optional<std::size_t> Node<TDerived, TTreeGrouping, TFrontend, CharType>::text_index_in_parent() const
    {
        if (parent_ == nullptr)
        {
            return std::nullopt;
        }

        return offset_->text_index_in_parent();
    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    void Node<TDerived, TTreeGrouping, TFrontend, CharType>::detach_from_parent()
    {

    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    void
    Node<TDerived, TTreeGrouping, TFrontend, CharType>::attach_to_parent(InternalType &new_parent,
                                                                         ParentOffsets::ParentOffset new_parent_offset)
    {

    }

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename CharType>
        requires std::is_base_of_v<Kernels::LanguageFrontend, TFrontend>
    void Node<TDerived, TTreeGrouping, TFrontend, CharType>::unsafe_forcibly_attach_to_parent(
        InternalType &new_parent, ParentOffsets::ParentOffset new_parent_offset)
    {
    }
} // namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Base
