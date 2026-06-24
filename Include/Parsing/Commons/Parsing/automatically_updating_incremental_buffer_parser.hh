
#ifndef SOFTOMNILIB_AUTOMATICALLY_UPDATING_INCREMENTAL_PARSER_HH
#define SOFTOMNILIB_AUTOMATICALLY_UPDATING_INCREMENTAL_PARSER_HH

#include <Include/Parsing/Commons/Parsing/incremental_parser.hh>
#include <Include/Parsing/Commons/Parsing/automatically_updating_buffer_parser.hh>

namespace SoftOmni::Parsing::Commons::Parsing
{
    template <typename TFrontend, typename TBufferCharType>
    class AutomaticallyUpdatingIncrementalBufferParser : AutomaticallyUpdatingBufferParser<TFrontend, TBufferCharType>,
        virtual IncrementalParser<TFrontend>
    {
    protected:
        explicit AutomaticallyUpdatingIncrementalBufferParser(const TFrontend& frontend, const Buffers::Buffer<TBufferCharType>& buffer);
    };
} // SoftOmni::Parsing::Commons::Parsing

#endif //SOFTOMNILIB_AUTOMATICALLY_UPDATING_INCREMENTAL_PARSER_HH
