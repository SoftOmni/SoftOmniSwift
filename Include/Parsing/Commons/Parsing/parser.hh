
#ifndef SOFTOMNILIB_PARSER_HH
#define SOFTOMNILIB_PARSER_HH
#include <optional>

namespace SoftOmni::Parsing::Commons::Parsing
{
    enum class ParserState
    {
        NOT_PARSED,
        PARSING,
        INCREMENTALLY_REPARSING,
        OUT_OF_DATE,
        OUT_OF_DATE_LOCALLY,
        REPARSING,
        PARSED
    };

    template <typename TFrontend>
    class Parser
    {
        ParserState parser_state_;

        TFrontend *frontend_;

    protected:
        explicit Parser(const TFrontend& frontend);

    public:
        virtual ParserState parser_state() const;

        std::optional<std::reference_wrapper<TFrontend>> frontend() const;

        [[nodiscard]] bool is_parsed() const;

        [[nodiscard]] bool is_updating() const;

        virtual void parse() = 0;

        virtual void reparse() = 0;
    };
}

#endif //SOFTOMNILIB_PARSER_HH
