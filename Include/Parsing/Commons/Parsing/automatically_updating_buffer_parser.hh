
#ifndef SOFTOMNILIB_AUTOMATICALLY_UPDATING_BUFFER_PARSER_HH
#define SOFTOMNILIB_AUTOMATICALLY_UPDATING_BUFFER_PARSER_HH

#include <Include/Parsing/Commons/Parsing/buffer_parser.hh>
#include <memory>

namespace SoftOmni::Parsing::Commons::Parsing
{
    template <typename TFrontend, typename TBufferCharType>
    class AutomaticallyUpdatingBufferParser : public BufferParser<TFrontend, TBufferCharType>
    {
    protected:
        AutomaticallyUpdatingBufferParser(const TFrontend& frontend, const Buffers::Buffer<TBufferCharType>& buffer);

    public:
        class AutomaticallyUpdatingParserBuffer final: public Buffers::Buffer<TBufferCharType>
        {
            std::unique_ptr<Buffers::Buffer<TBufferCharType>> buffer_;

        protected:
            explicit AutomaticallyUpdatingParserBuffer(std::unique_ptr<Buffers::Buffer<TBufferCharType>> containing_buffer);

        public:
            [[nodiscard]] Buffers::Buffer<TBufferCharType>::elemental_char_access_type operator[](std::size_t index) const;

            [[nodiscard]] TBufferCharType& operator[](std::size_t index);

            ~AutomaticallyUpdatingParserBuffer();
        };
    };
}

#endif //SOFTOMNILIB_AUTOMATICALLY_UPDATING_BUFFER_PARSER_HH
