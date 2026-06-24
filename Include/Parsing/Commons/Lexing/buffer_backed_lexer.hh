
#ifndef SOFTOMNILIB_BUFFER_BACKED_LEXER_HH
#define SOFTOMNILIB_BUFFER_BACKED_LEXER_HH

#include <Include/Parsing/Commons/Buffers/buffer.hh>
#include <Include/Parsing/Commons/Lexing/lexer.hh>
#include <memory>

namespace SoftOmni::Parsing::Commons::Lexing
{
    template <typename TToken, typename TBufferCharType>
    class BufferBackedLexer : Lexer<TToken>
    {
        std::unique_ptr<Buffers::Buffer<TBufferCharType>> buffer_;

    protected:
        explicit BufferBackedLexer(std::unique_ptr<Buffers::Buffer<TBufferCharType>> buffer);

    public:
        const Buffers::Buffer<TBufferCharType>& buffer() const;
    };
}


#endif //SOFTOMNILIB_BUFFER_BACKED_LEXER_HH
