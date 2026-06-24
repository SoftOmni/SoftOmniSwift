
#ifndef SOFTOMNILIB_BUFFER_PARSER_HH
#define SOFTOMNILIB_BUFFER_PARSER_HH


#include <Include/Parsing/Commons/Buffers/buffer.hh>
#include <Include/Parsing/Commons/Parsing/parser.hh>

namespace SoftOmni::Parsing::Commons::Parsing
{
    template <typename TFrontend, typename TBufferCharType>
    class BufferParser : Parser<TFrontend>
    {
    protected:
        Buffers::Buffer<TBufferCharType> *buffer_;

    public:
        explicit BufferParser(const TFrontend& frontend, const Buffers::Buffer<TBufferCharType>& buffer);

        const Buffers::Buffer<TBufferCharType>& buffer() const;

        virtual ~BufferParser();
    };
}

#endif //SOFTOMNILIB_BUFFER_PARSER_HH
