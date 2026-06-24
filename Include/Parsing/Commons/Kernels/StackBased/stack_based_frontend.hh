
#ifndef SOFTOMNILIB_STACK_BASED_FRONTEND_HH
#define SOFTOMNILIB_STACK_BASED_FRONTEND_HH

#include <Include/Parsing/Commons/Kernels/language_frontend.hh>
#include <vector>

namespace SoftOmni::Parsing::Commons::Kernels::StackBased
{
    template <typename TLanguagePrimitives> // TODO: add constraints to this later
    class StackBasedLanguageFrontend: public LanguageFrontend
    {
        using iterator = std::optional<std::vector<TLanguagePrimitives>>::const_iterator;

        std::vector<TLanguagePrimitives> stack_;

    protected:
        explicit StackBasedLanguageFrontend(const std::string &name, const std::string &language);

        explicit StackBasedLanguageFrontend(const std::string &name, Language &language);

        StackBasedLanguageFrontend(std::string name, std::string display_name, const std::string &language);

        StackBasedLanguageFrontend(std::string name, std::string display_name, Language& language);

    public:
        [[nodiscard]] std::size_t stack_size() const;

        [[nodiscard]] bool is_empty() const;

        enum class StackIndexReference
        {
            BottomIsStart,
            TopIsStart
        };

        [[nodiscard]] iterator get_iterator_at_position(std::size_t index, StackIndexReference index_reference = StackIndexReference::BottomIsStart) const;

        [[nodiscard]] const TLanguagePrimitives& get_primitive_at_position(std::size_t index, StackIndexReference index_reference = StackIndexReference::BottomIsStart) const;

        [[nodiscard]] std::optional<std::size_t> operator[](std::size_t index, StackIndexReference index_reference = StackIndexReference::BottomIsStart) const;

        [[nodiscard]] std::optional<TLanguagePrimitives> peek() const;

        [[nodiscard]] std::optional<TLanguagePrimitives> top() const;

        [[nodiscard]] std::vector<TLanguagePrimitives>::const_iterator cbegin_bottom() const;

        [[nodiscard]] std::vector<TLanguagePrimitives>::const_iterator cend_bottom() const;

        [[nodiscard]] std::vector<TLanguagePrimitives>::const_iterator cbegin_top() const;

        [[nodiscard]] std::vector<TLanguagePrimitives>::const_iterator cend_top() const;

    protected:
        template <typename... Args> requires (std::convertible_to<Args, TLanguagePrimitives> && ...)
        void push(Args&&... args);

        TLanguagePrimitives pop();

        std::vector<TLanguagePrimitives> pop(std::size_t number_of_primitives);

        void pop_and_discard(std::size_t number_of_primitives);

        void swap(std::size_t first_element_index, std::size_t second_element_index);

        bool reserve(std::size_t new_capacity);

        void resize(std::size_t new_capacity);

        void remove_at(std::size_t index, StackIndexReference index_reference = StackIndexReference::BottomIsStart);

        void insert_at(std::size_t index, TLanguagePrimitives primitive);

        // TODO: Add stack modifications
    };
}

#include "stack_based_frontend.hxx"

#endif //SOFTOMNILIB_STACK_BASED_FRONTEND_HH
