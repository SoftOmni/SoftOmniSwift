
#ifndef SOFTOMNILIB_LINE_COLUMN_AWARE_POSITION_HH
#define SOFTOMNILIB_LINE_COLUMN_AWARE_POSITION_HH

#include <Include/Parsing/Commons/Lexing/Tokens/Positioning/position.hh>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
{
    class LineColumnAwarePosition: virtual Position
    {
    protected:
        std::size_t line_;

        std::size_t column_;

    public:
        explicit LineColumnAwarePosition(std::size_t overall_position, std::size_t line, std::size_t column, OperatorBehavior operator_behavior = OperatorBehavior::CheckedSafe);

        [[nodiscard]] std::size_t overall_position() const;

        void set_overall_position(std::size_t new_overall_position);

        [[nodiscard]] std::size_t line() const;

        void set_line(std::size_t new_line);

        [[nodiscard]] std::size_t column() const;

        void set_column(std::size_t new_column);

        [[nodiscard]] OperatorBehavior operator_behavior() const;

        void set_operator_behavior(OperatorBehavior new_operator_behavior);

        bool increment_line(bool reset_column = true);

        bool increment_line_by(std::size_t amount, bool reset_column = true);

        bool increment_line_and_set_column(std::size_t new_column);

        bool increment_line_by_and_set_column(std::size_t amount, std::size_t new_column);

        bool increment_column();

        bool increment_column_by(std::size_t amount);

        bool increment_column_by_and_set_line(std::size_t amount, std::size_t new_line);

        bool increment_line_checked(bool reset_column = true);

        bool increment_line_by_checked(std::size_t amount, bool reset_column = true);

        bool increment_line_and_set_column_checked(std::size_t new_column);

        bool increment_line_by_and_set_column_checked(std::size_t amount, std::size_t new_column);

        bool increment_column_checked();

        bool increment_column_by_checked(std::size_t amount);

        bool increment_column_by_and_set_line_checked(std::size_t amount, std::size_t new_line);

        bool increment_line_maximalize(bool reset_column = true);

        bool increment_line_by_maximalize(std::size_t amount, bool reset_column = true);

        bool increment_line_and_set_column_maximalize(std::size_t new_column);

        bool increment_line_by_and_set_column_maximalize(std::size_t amount, std::size_t new_column);

        bool increment_column_maximalize();

        bool increment_column_by_maximalize(std::size_t amount);

        bool increment_column_by_and_set_line_maximalize(std::size_t amount, std::size_t new_line);

        bool unsafe_unchecked_increment_line(bool reset_column = true);

        bool unsafe_unchecked_increment_line_by(std::size_t amount, bool reset_column = true);

        bool unsafe_unchecked_increment_line_and_set_column(std::size_t new_column);

        bool unsafe_unchecked_increment_line_by_and_set_column(std::size_t amount, std::size_t new_column);

        bool unsafe_unchecked_increment_column();

        bool unsafe_unchecked_increment_column_by(std::size_t amount);

        bool unsafe_unchecked_increment_column_by_and_set_line(std::size_t amount, std::size_t new_line);

        bool decrement_line(bool reset_column = true);

        bool decrement_line_by(std::size_t amount, bool reset_column = true);

        bool decrement_line_and_set_column(std::size_t new_column);

        bool decrement_line_by_and_set_column(std::size_t amount, std::size_t new_column);

        bool decrement_column();

        bool decrement_column_by(std::size_t amount);

        bool decrement_column_by_and_set_line(std::size_t amount, std::size_t new_line);

        bool decrement_line_checked(bool reset_column = true);

        bool decrement_line_by_checked(std::size_t amount, bool reset_column = true);

        bool decrement_line_and_set_column_checked(std::size_t new_column);

        bool decrement_line_by_and_set_column_checked(std::size_t amount, std::size_t new_column);

        bool decrement_column_checked();

        bool decrement_column_by_checked(std::size_t amount);

        bool decrement_column_by_and_set_line_checked(std::size_t amount, std::size_t new_line);

        bool decrement_line_minimalize(bool reset_column = true);

        bool decrement_line_by_minimalize(std::size_t amount, bool reset_column = true);

        bool decrement_line_and_set_column_minimalize(std::size_t new_column);

        bool decrement_line_by_and_set_column_minimalize(std::size_t amount, std::size_t new_column);

        bool decrement_column_minimalize();

        bool decrement_column_by_minimalize(std::size_t amount);

        bool decrement_column_by_and_set_line_minimalize(std::size_t amount, std::size_t new_line);

        bool unsafe_unchecked_decrement_line(bool reset_column = true);

        bool unsafe_unchecked_decrement_line_by(std::size_t amount, bool reset_column = true);

        bool unsafe_unchecked_decrement_line_and_set_column(std::size_t new_column);

        bool unsafe_unchecked_decrement_line_by_and_set_column(std::size_t amount, std::size_t new_column);

        bool unsafe_unchecked_decrement_column();

        bool unsafe_unchecked_decrement_column_by(std::size_t amount);

        bool unsafe_unchecked_decrement_column_by_and_set_line(std::size_t amount, std::size_t new_line);
    };
}

#endif //SOFTOMNILIB_LINE_COLUMN_AWARE_POSITION_HH
