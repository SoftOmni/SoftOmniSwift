

#ifndef SOFTOMNILIB_STACK_BASED_FRONTEND_HXX
#define SOFTOMNILIB_STACK_BASED_FRONTEND_HXX

namespace SoftOmni::Parsing::Commons::Kernels::StackBased
{
    template <typename TLanguagePrimitives>
    std::size_t StackBasedLanguageFrontend<TLanguagePrimitives>::stack_size() const
    {
        return stack_.size();
    }

    template <typename TLanguagePrimitives>
    bool StackBasedLanguageFrontend<TLanguagePrimitives>::is_empty() const
    {
        return stack_.empty();
    }

    template <typename TLanguagePrimitives>
    typename StackBasedLanguageFrontend<TLanguagePrimitives>::iterator
    StackBasedLanguageFrontend<TLanguagePrimitives>::get_iterator_at_position(std::size_t index,
                                                                              StackIndexReference index_reference) const
    {
        if (index >= stack_size())
        {
            return std::nullopt;
        }

        switch (index_reference)
        {
        case StackIndexReference::BottomIsStart:
                break;
        default:;
        }
    }

    template <typename TLanguagePrimitives>
    const TLanguagePrimitives &StackBasedLanguageFrontend<TLanguagePrimitives>::get_primitive_at_position(
        std::size_t index, StackIndexReference index_reference) const
    {
    }
    template <typename TLanguagePrimitives>
    std::optional<std::size_t>
    StackBasedLanguageFrontend<TLanguagePrimitives>::operator[](std::size_t index,
                                                                StackIndexReference index_reference) const
    {
    }
    template <typename TLanguagePrimitives>
    std::optional<TLanguagePrimitives> StackBasedLanguageFrontend<TLanguagePrimitives>::peek() const
    {
    }
    template <typename TLanguagePrimitives>
    std::optional<TLanguagePrimitives> StackBasedLanguageFrontend<TLanguagePrimitives>::top() const
    {
    }
    template <typename TLanguagePrimitives>
    typename std::vector<TLanguagePrimitives>::const_iterator
    StackBasedLanguageFrontend<TLanguagePrimitives>::cbegin_bottom() const
    {
    }
    template <typename TLanguagePrimitives>
    typename std::vector<TLanguagePrimitives>::const_iterator
    StackBasedLanguageFrontend<TLanguagePrimitives>::cend_bottom() const
    {
    }
    template <typename TLanguagePrimitives>
    typename std::vector<TLanguagePrimitives>::const_iterator
    StackBasedLanguageFrontend<TLanguagePrimitives>::cbegin_top() const
    {
    }
    template <typename TLanguagePrimitives>
    typename std::vector<TLanguagePrimitives>::const_iterator
    StackBasedLanguageFrontend<TLanguagePrimitives>::cend_top() const
    {
    }
    template <typename TLanguagePrimitives>
    TLanguagePrimitives StackBasedLanguageFrontend<TLanguagePrimitives>::pop()
    {
    }
    template <typename TLanguagePrimitives>
    std::vector<TLanguagePrimitives>
    StackBasedLanguageFrontend<TLanguagePrimitives>::pop(std::size_t number_of_primitives)
    {
    }
    template <typename TLanguagePrimitives>
    void StackBasedLanguageFrontend<TLanguagePrimitives>::pop_and_discard(std::size_t number_of_primitives)
    {
    }
    template <typename TLanguagePrimitives>
    void StackBasedLanguageFrontend<TLanguagePrimitives>::swap(std::size_t first_element_index,
                                                               std::size_t second_element_index)
    {
    }
    template <typename TLanguagePrimitives>
    bool StackBasedLanguageFrontend<TLanguagePrimitives>::reserve(std::size_t new_capacity)
    {
    }
    template <typename TLanguagePrimitives>
    void StackBasedLanguageFrontend<TLanguagePrimitives>::resize(std::size_t new_capacity)
    {
    }
    template <typename TLanguagePrimitives>
    void StackBasedLanguageFrontend<TLanguagePrimitives>::remove_at(std::size_t index,
                                                                    StackIndexReference index_reference)
    {
    }
    template <typename TLanguagePrimitives>
    void StackBasedLanguageFrontend<TLanguagePrimitives>::insert_at(std::size_t index, TLanguagePrimitives primitive)
    {
    }
}

#endif //SOFTOMNILIB_STACK_BASED_FRONTEND_HXX
