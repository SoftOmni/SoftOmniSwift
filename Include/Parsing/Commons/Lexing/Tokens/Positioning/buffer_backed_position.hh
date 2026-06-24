

#ifndef SOFTOMNILIB_BUFFER_BACKED_POSITION_HH
#define SOFTOMNILIB_BUFFER_BACKED_POSITION_HH

#include <Include/Parsing/Commons/Buffers/buffer.hh>
#include <Include/Parsing/Commons/Lexing/Tokens/Positioning/position.hh>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
{
    template <typename TCharType>
    class BufferBackedPosition: virtual public Position
    {
        std::reference_wrapper<Buffers::Buffer<TCharType>> buffer_;

        // consider moves and buffer memory
        // consider a registry linking this to a buffer
    public:
        explicit BufferBackedPosition(OperatorBehavior operator_behavior = OperatorBehavior::CheckedSafe);

        explicit BufferBackedPosition(std::size_t position, OperatorBehavior operator_behavior = OperatorBehavior::CheckedSafe);

        [[nodiscard]] Buffers::Buffer<TCharType>& buffer() const;

        [[nodiscard]] bool is_position_still_valid_in_buffer() const;

        virtual ~BufferBackedPosition();
    };
}

#endif //SOFTOMNILIB_BUFFER_BACKED_POSITION_HH
