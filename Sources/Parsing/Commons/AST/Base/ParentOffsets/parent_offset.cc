
#include <Parsing/Commons/AST/Base/ParentOffsets/parent_offset.hh>

namespace SoftOmni::Parsing::Commons::AST::Base::ParentOffsets
{
    ParentOffset::ParentOffset(const std::size_t index_in_parent, const std::size_t text_index_in_parent)
        : index_in_parent_(index_in_parent), text_index_in_parent_(text_index_in_parent)
    {
    }

    ParentOffset::ParentOffset(const ParentOffset &other) = default;

    ParentOffset::ParentOffset(ParentOffset &&other) noexcept = default;

    ParentOffset& ParentOffset::operator=(const ParentOffset &other)
    {
        if (this == &other)
            return *this;
        index_in_parent_ = other.index_in_parent_;
        text_index_in_parent_ = other.text_index_in_parent_;
        return *this;
    }

    ParentOffset& ParentOffset::operator=(ParentOffset &&other) noexcept
    {
        if (this == &other)
            return *this;
        index_in_parent_ = other.index_in_parent_;
        text_index_in_parent_ = other.text_index_in_parent_;
        return *this;
    }

    std::size_t ParentOffset::index_in_parent() const
    {
        return index_in_parent_;
    }

    std::size_t ParentOffset::text_index_in_parent() const
    {
        return text_index_in_parent_;
    }

    void ParentOffset::set_index_in_parent(const std::size_t new_index_in_parent)
    {
        index_in_parent_ = new_index_in_parent;
    }

    void ParentOffset::set_text_index_in_parent(const std::size_t new_text_index_in_parent)
    {
        text_index_in_parent_ = new_text_index_in_parent;
    }

    int ParentOffset::operator<=>(const ParentOffset &other) const
    {
        if (index_in_parent_ < other.index_in_parent_)
        {
            return -1;
        }

        if (index_in_parent_ > other.index_in_parent_)
        {
            return 1;
        }

        if (text_index_in_parent_ < other.text_index_in_parent_)
        {
            return -1;
        }

        if (text_index_in_parent_ > other.text_index_in_parent_)
        {
            return 1;
        }

        return 0;
    }
} // namespace SoftOmni::Parsing::Commons::AST::Base::ParentOffsets
