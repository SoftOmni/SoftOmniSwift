
#ifndef SOFTOMNILIB_BUFFER_HH
#define SOFTOMNILIB_BUFFER_HH
#include <cstddef>
#include <string>
#include <type_traits>

namespace SoftOmni::Parsing::Commons::Buffers
{
    template <typename CharType>
    class Buffer
    {
        std::size_t size_;

    protected:
        using elemental_char_access_type = std::conditional_t<sizeof(CharType) <= sizeof(std::size_t), CharType, const CharType&>;

        explicit Buffer(std::size_t size);

    public:
        [[nodiscard]] std::size_t size() const;

        [[nodiscard]] virtual elemental_char_access_type operator[](std::size_t index) const = 0;

        [[nodiscard]] virtual CharType& operator[](std::size_t index) = 0;

        [[nodiscard]] virtual std::size_t get_size_of_element_at_index(std::size_t index) const;

        [[nodiscard]] virtual std::size_t get_total_memory_footprint() const;

        [[nodiscard]] virtual bool contains(elemental_char_access_type character) const;

        template <typename TOtherCharType>
        [[nodiscard]] bool contains(std::basic_string<TOtherCharType> string) const;

        [[nodiscard]] virtual bool contains_as_text(const Buffer& buffer) const;

        virtual ~Buffer();
    };
}

#endif //SOFTOMNILIB_BUFFER_HH
