
#ifndef SOFTOMNILIB_INCREMENTAL_PARSER_HH
#define SOFTOMNILIB_INCREMENTAL_PARSER_HH

#include <Include/Parsing/Commons/Parsing/parser.hh>

namespace SoftOmni::Parsing::Commons::Parsing
{
    template <typename TFrontend>
    class IncrementalParser : virtual Parser<TFrontend>
    {
    public:
        explicit IncrementalParser(const TFrontend& frontend);

        virtual void reparse_out_of_date() = 0;
    };
}

#endif //SOFTOMNILIB_INCREMENTAL_PARSER_HH
