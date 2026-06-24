#include <Parsing/Commons/Lexing/Tokens/Positioning/line_column_aware_position.hh>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
{
    LineColumnAwarePosition::LineColumnAwarePosition(const std::size_t overall_position, const std::size_t line,
                                                     const std::size_t column,
                                                     const OperatorBehavior operator_behavior)
                                                         : Position(overall_position, operator_behavior), line_(line), column_(column)
    {}

    std::size_t LineColumnAwarePosition::overall_position() const
    {
        return position_;
    }

    void LineColumnAwarePosition::set_overall_position(const std::size_t new_overall_position)
    {
        position_ = new_overall_position;
    }

    std::size_t LineColumnAwarePosition::line() const
    {
        return line_;
    }

    void LineColumnAwarePosition::set_line(const std::size_t new_line)
    {
        line_ = new_line;
    }

    std::size_t LineColumnAwarePosition::column() const
    {
        return column_;
    }

    void LineColumnAwarePosition::set_column(const std::size_t new_column)
    {
        column_ = new_column;
    }

    OperatorBehavior LineColumnAwarePosition::operator_behavior() const
    {
        return Position::operator_behavior();
    }

    void LineColumnAwarePosition::set_operator_behavior(const OperatorBehavior new_operator_behavior)
    {
        Position::set_operator_behavior(new_operator_behavior);
    }

    bool LineColumnAwarePosition::increment_line(const bool reset_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return increment_line_checked(reset_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return increment_line_maximalize(reset_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_increment_line(reset_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::increment_line_by(const std::size_t amount, const bool reset_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return increment_line_by_checked(amount, reset_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return increment_line_by_maximalize(amount, reset_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_increment_line_by(amount, reset_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::increment_line_and_set_column(const std::size_t new_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return increment_line_and_set_column_checked(new_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return increment_line_and_set_column_maximalize(new_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_increment_line_and_set_column(new_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::increment_line_by_and_set_column(const std::size_t amount,
                                                                   const std::size_t new_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return increment_line_by_and_set_column_checked(amount, new_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return increment_line_by_and_set_column_maximalize(amount, new_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_increment_line_by_and_set_column(amount, new_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::increment_column()
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return increment_column_checked();
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return increment_column_maximalize();
        case UncheckedUnsafe:
            return unsafe_unchecked_increment_column();
        }

        return false;
    }

    bool LineColumnAwarePosition::increment_column_by(const std::size_t amount)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return increment_column_by_checked(amount);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return increment_column_by_maximalize(amount);
        case UncheckedUnsafe:
            return unsafe_unchecked_increment_column_by(amount);
        }

        return false;
    }

    bool LineColumnAwarePosition::increment_column_by_and_set_line(const std::size_t amount, const std::size_t new_line)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return increment_column_by_and_set_line_checked(amount, new_line);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return increment_column_by_and_set_line_maximalize(amount, new_line);
        case UncheckedUnsafe:
            return unsafe_unchecked_increment_column_by_and_set_line(amount, new_line);
        }

        return false;
    }

    bool LineColumnAwarePosition::increment_line_checked(const bool reset_column)
    {
        if (line_ == SIZE_MAX)
        {
            return false;
        }

        return unsafe_unchecked_increment_line(reset_column);
    }

    bool LineColumnAwarePosition::increment_line_by_checked(const std::size_t amount, const bool reset_column)
    {
        if (SIZE_MAX - line_ < amount)
        {
            return false;
        }

        return unsafe_unchecked_increment_line_by(amount, reset_column);
    }

    bool LineColumnAwarePosition::increment_line_and_set_column_checked(const std::size_t new_column)
    {
        if (line_ == SIZE_MAX)
        {
            return false;
        }

        return unsafe_unchecked_increment_line_and_set_column(new_column);
    }

    bool LineColumnAwarePosition::increment_line_by_and_set_column_checked(const std::size_t amount,
                                                                           const std::size_t new_column)
    {
        if (SIZE_MAX - line_ < amount)
        {
            return false;
        }

        return unsafe_unchecked_increment_line_by_and_set_column(amount, new_column);
    }

    bool LineColumnAwarePosition::increment_column_checked()
    {
        if (column_ == SIZE_MAX)
        {
            return false;
        }

        return unsafe_unchecked_increment_column();
    }

    bool LineColumnAwarePosition::increment_column_by_checked(const std::size_t amount)
    {
        if (SIZE_MAX - column_ < amount)
        {
            return false;
        }

        return unsafe_unchecked_increment_column_by(amount);
    }

    bool LineColumnAwarePosition::increment_column_by_and_set_line_checked(const std::size_t amount,
                                                                           const std::size_t new_line)
    {
        if (SIZE_MAX - column_ < amount)
        {
            return false;
        }

        return unsafe_unchecked_increment_column_by_and_set_line(amount, new_line);
    }

    bool LineColumnAwarePosition::increment_line_maximalize(const bool reset_column)
    {
        return increment_line_checked(reset_column);
    }

    bool LineColumnAwarePosition::increment_line_by_maximalize(const std::size_t amount, const bool reset_column)
    {
        if (SIZE_MAX - line_ < amount)
        {
            line_ = SIZE_MAX;
            if (reset_column)
            {
                column_ = 0;
            }

            return false;
        }

        return unsafe_unchecked_increment_line_by(amount, reset_column);
    }

    bool LineColumnAwarePosition::increment_line_and_set_column_maximalize(const std::size_t new_column)
    {
        return increment_line_and_set_column_checked(new_column);
    }

    bool LineColumnAwarePosition::increment_line_by_and_set_column_maximalize(const std::size_t amount,
                                                                              const std::size_t new_column)
    {
        if (SIZE_MAX - line_ < amount)
        {
            line_ = SIZE_MAX;
            column_ = new_column;

            return false;
        }

        return unsafe_unchecked_increment_line_by_and_set_column(amount, new_column);
    }

    bool LineColumnAwarePosition::increment_column_maximalize()
    {
        return increment_column_checked();
    }

    bool LineColumnAwarePosition::increment_column_by_maximalize(const std::size_t amount)
    {
        if (SIZE_MAX - column_ < amount)
        {
            column_ = SIZE_MAX;
            return false;
        }

        return unsafe_unchecked_increment_column_by(amount);
    }

    bool LineColumnAwarePosition::increment_column_by_and_set_line_maximalize(const std::size_t amount,
                                                                              const std::size_t new_line)
    {
        if (SIZE_MAX - column_ < amount)
        {
            column_ = SIZE_MAX;
            line_ = new_line;

            return false;
        }

        return unsafe_unchecked_increment_column_by_and_set_line(amount, new_line);
    }

    bool LineColumnAwarePosition::unsafe_unchecked_increment_line(const bool reset_column)
    {
        line_++;
        if (reset_column)
        {
            column_ = 0;
        }

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_increment_line_by(const std::size_t amount, const bool reset_column)
    {
        line_ += amount;
        if (reset_column)
        {
            column_ = 0;
        }

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_increment_line_and_set_column(const std::size_t new_column)
    {
        line_++;
        column_ = new_column;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_increment_line_by_and_set_column(const std::size_t amount,
                                                                                    const std::size_t new_column)
    {
        line_ += amount;
        column_ += new_column;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_increment_column()
    {
        column_++;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_increment_column_by(const std::size_t amount)
    {
        column_ += amount;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_increment_column_by_and_set_line(const std::size_t amount,
                                                                                    const std::size_t new_line)
    {
        column_ += amount;
        line_ = new_line;

        return true;
    }

    bool LineColumnAwarePosition::decrement_line(const bool reset_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return decrement_line_checked(reset_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return decrement_line_minimalize(reset_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_decrement_line(reset_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::decrement_line_by(const std::size_t amount, const bool reset_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return decrement_line_by_checked(amount, reset_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return decrement_line_by_minimalize(amount, reset_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_decrement_line_by(amount, reset_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::decrement_line_and_set_column(const std::size_t new_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return decrement_line_and_set_column_checked(new_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return decrement_line_and_set_column_minimalize(new_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_decrement_line_and_set_column(new_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::decrement_line_by_and_set_column(const std::size_t amount,
                                                                   const std::size_t new_column)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return decrement_line_by_and_set_column_checked(amount, new_column);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return decrement_line_by_and_set_column_minimalize(amount, new_column);
        case UncheckedUnsafe:
            return unsafe_unchecked_decrement_line_by_and_set_column(amount, new_column);
        }

        return false;
    }

    bool LineColumnAwarePosition::decrement_column()
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return decrement_column_checked();
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return decrement_column_minimalize();
        case UncheckedUnsafe:
            return unsafe_unchecked_decrement_column();
        }

        return false;
    }

    bool LineColumnAwarePosition::decrement_column_by(const std::size_t amount)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return decrement_column_by_checked(amount);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return decrement_column_by_minimalize(amount);
        case UncheckedUnsafe:
            return unsafe_unchecked_decrement_column_by(amount);
        }

        return false;
    }

    bool LineColumnAwarePosition::decrement_column_by_and_set_line(const std::size_t amount, const std::size_t new_line)
    {
        using enum OperatorBehavior;

        switch (operator_behavior_)
        {
        case CheckedSafe:
            return decrement_column_by_and_set_line_checked(amount, new_line);
        case CheckedSafeMaximalizeAndMinimalizeValue:
            return decrement_column_by_and_set_line_minimalize(amount, new_line);
        case UncheckedUnsafe:
            return unsafe_unchecked_decrement_column_by_and_set_line(amount, new_line);
        }

        return false;
    }

    bool LineColumnAwarePosition::decrement_line_checked(const bool reset_column)
    {
        if (line_ == 0)
        {
            return false;
        }

        return unsafe_unchecked_decrement_line(reset_column);
    }

    bool LineColumnAwarePosition::decrement_line_by_checked(const std::size_t amount, const bool reset_column)
    {
        if (amount > line_)
        {
            return false;
        }

        return unsafe_unchecked_decrement_line_by(amount, reset_column);
    }

    bool LineColumnAwarePosition::decrement_line_and_set_column_checked(const std::size_t new_column)
    {
        if (line_ == 0)
        {
            return false;
        }

        return unsafe_unchecked_decrement_line_and_set_column(new_column);
    }

    bool LineColumnAwarePosition::decrement_line_by_and_set_column_checked(const std::size_t amount,
                                                                           const std::size_t new_column)
    {
        if (amount > line_)
        {
            return false;
        }

        return unsafe_unchecked_decrement_line_by_and_set_column(amount, new_column);
    }

    bool LineColumnAwarePosition::decrement_column_checked()
    {
        if (column_ == 0)
        {
            return false;
        }

        return unsafe_unchecked_decrement_column();
    }

    bool LineColumnAwarePosition::decrement_column_by_checked(const std::size_t amount)
    {
        if (amount > column_)
        {
            return false;
        }

        return unsafe_unchecked_decrement_column_by(amount);
    }

    bool LineColumnAwarePosition::decrement_column_by_and_set_line_checked(const std::size_t amount,
                                                                           const std::size_t new_line)
    {
        if (amount > column_)
        {
            return false;
        }

        return unsafe_unchecked_decrement_column_by_and_set_line(amount, new_line);
    }

    bool LineColumnAwarePosition::decrement_line_minimalize(const bool reset_column)
    {
        return decrement_line_checked(reset_column);
    }

    bool LineColumnAwarePosition::decrement_line_by_minimalize(const std::size_t amount, const bool reset_column)
    {
        if (amount > line_)
        {
            line_ = 0;
            if (reset_column)
            {
                column_ = 0;
            }

            return false;
        }

        return unsafe_unchecked_decrement_line_by(amount, reset_column);
    }

    bool LineColumnAwarePosition::decrement_line_and_set_column_minimalize(const std::size_t new_column)
    {
        return decrement_line_and_set_column_checked(new_column);
    }

    bool LineColumnAwarePosition::decrement_line_by_and_set_column_minimalize(const std::size_t amount,
                                                                              const std::size_t new_column)
    {
        if (amount > line_)
        {
            line_ = 0;
            column_ = new_column;

            return false;
        }

        return unsafe_unchecked_decrement_line_by_and_set_column(amount, new_column);
    }

    bool LineColumnAwarePosition::decrement_column_minimalize()
    {
        return decrement_column_checked();
    }

    bool LineColumnAwarePosition::decrement_column_by_minimalize(const std::size_t amount)
    {
        if (amount > column_)
        {
            column_ = 0;
            return false;
        }

        return unsafe_unchecked_decrement_column_by(amount);
    }

    bool LineColumnAwarePosition::decrement_column_by_and_set_line_minimalize(const std::size_t amount,
                                                                              const std::size_t new_line)
    {
        if (amount > column_)
        {
            column_ = 0;
            line_ = new_line;

            return false;
        }

        return unsafe_unchecked_decrement_column_by_and_set_line(amount, new_line);
    }

    bool LineColumnAwarePosition::unsafe_unchecked_decrement_line(const bool reset_column)
    {
        line_--;
        if (reset_column)
        {
            column_ = 0;
        }

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_decrement_line_by(const std::size_t amount, const bool reset_column)
    {
        line_ -= amount;
        if (reset_column)
        {
            column_ = 0;
        }

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_decrement_line_and_set_column(const std::size_t new_column)
    {
        line_--;
        column_ = new_column;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_decrement_line_by_and_set_column(const std::size_t amount,
                                                                                    const std::size_t new_column)
    {
        line_ -= amount;
        column_ = new_column;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_decrement_column()
    {
        column_--;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_decrement_column_by(const std::size_t amount)
    {
        column_ -= amount;

        return true;
    }

    bool LineColumnAwarePosition::unsafe_unchecked_decrement_column_by_and_set_line(const std::size_t amount,
                                                                                    const std::size_t new_line)
    {
        column_ -= amount;
        line_ = new_line;

        return true;
    }
} // namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
