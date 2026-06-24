

#ifndef SOFTOMNILIB_BUFFER_BACKED_LINE_COLUMN_AWARE_POSITION_HH
#define SOFTOMNILIB_BUFFER_BACKED_LINE_COLUMN_AWARE_POSITION_HH

#include <Include/Parsing/Commons/Lexing/Tokens/Positioning/line_column_aware_position.hh>
#include <Include/Parsing/Commons/Lexing/Tokens/Positioning/buffer_backed_position.hh>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
{
    template <typename TCharType>
    class BufferBackedLineColumnAwarePosition: public LineColumnAwarePosition, BufferBackedPosition<TCharType>
    {
    public:
        explicit BufferBackedLineColumnAwarePosition(const Buffers::Buffer<TCharType>& buffer, std::size_t overall_position, std::size_t line, std::size_t column,
        OperatorBehavior operator_behavior = OperatorBehavior::CheckedSafe);

        [[nodiscard]] Buffers::Buffer<TCharType>& buffer() const;

        [[nodiscard]] bool is_position_still_valid_in_buffer() const;

        virtual ~BufferBackedLineColumnAwarePosition();
    };
}

#endif //SOFTOMNILIB_BUFFER_BACKED_LINE_COLUMN_AWARE_POSITION_HH
