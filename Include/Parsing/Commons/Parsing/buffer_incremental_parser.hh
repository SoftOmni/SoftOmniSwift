
#ifndef SOFTOMNILIB_BUFFER_INCREMENTAL_PARSER_HH
#define SOFTOMNILIB_BUFFER_INCREMENTAL_PARSER_HH

#include <Include/Parsing/Commons/Parsing/buffer_parser.hh>
#include <Include/Parsing/Commons/Parsing/incremental_parser.hh>

namespace SoftOmni::Parsing::Commons::Parsing
{
    template <typename TFrontend, typename TBufferCharType>
    class BufferIncrementalParser : BufferParser<TFrontend, TBufferCharType>, virtual IncrementalParser<TFrontend>
    {
    public:
        explicit BufferIncrementalParser(const TFrontend& frontend, const Buffers::Buffer<TBufferCharType>& buffer);

        virtual ParserState parser_state(std::size_t start, std::size_t end) const;

        [[nodiscard]] bool is_parsed(std::size_t start, std::size_t end) const;

        virtual void reparse(std::size_t start, std::size_t end) = 0;
    };
}

#endif //SOFTOMNILIB_BUFFER_INCREMENTAL_PARSER_HH
