
#ifndef SOFTOMNILIB_PARENT_OFFSET_HH
#define SOFTOMNILIB_PARENT_OFFSET_HH
#include <cstddef>

namespace SoftOmni::Parsing::Commons::AST::Base::ParentOffsets
{
    class ParentOffset
    {
        std::size_t index_in_parent_;

        std::size_t text_index_in_parent_;

    public:
        explicit ParentOffset(std::size_t index_in_parent, std::size_t text_index_in_parent);

        ParentOffset(const ParentOffset& other);

        ParentOffset(ParentOffset &&other) noexcept;

        ParentOffset &operator=(const ParentOffset &other);

        ParentOffset &operator=(ParentOffset &&other) noexcept;

        [[nodiscard]] std::size_t index_in_parent() const;

        [[nodiscard]] std::size_t text_index_in_parent() const;

        void set_index_in_parent(std::size_t new_index_in_parent);

        void set_text_index_in_parent(std::size_t new_text_index_in_parent);

        friend bool operator==(const ParentOffset &lhs, const ParentOffset &rhs)
        {
            return lhs.index_in_parent_ == rhs.index_in_parent_ && lhs.text_index_in_parent_ == rhs.text_index_in_parent_;
        }

        int operator<=>(const ParentOffset &other) const;
    };
}

#endif //SOFTOMNILIB_PARENT_OFFSET_HH
