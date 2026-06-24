#include <Parsing/Commons/Lexing/Tokens/Positioning/position.hh>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
{
    Position::Position(const OperatorBehavior operator_behavior) : operator_behavior_(operator_behavior), position_(0)
    {
    }

    Position::Position(const std::size_t position, const OperatorBehavior operator_behavior) :
        operator_behavior_(operator_behavior), position_(position)
    {
    }

    OperatorBehavior Position::operator_behavior() const { return operator_behavior_; }

    void Position::set_operator_behavior(const OperatorBehavior new_operator_behavior)
    {
        using enum OperatorBehavior;

        if (new_operator_behavior != CheckedSafe &&
            new_operator_behavior != CheckedSafeMaximalizeAndMinimalizeValue &&
            new_operator_behavior != UncheckedUnsafe) [[unlikely]]
        {
            throw illegal_operator_behavior_exception(new_operator_behavior);
        }

        operator_behavior_ = new_operator_behavior;
    }

    std::size_t Position::position() const { return position_; }

    void Position::set_position(const std::size_t new_position) { position_ = new_position; }

    bool Position::increment()
    {
        switch (operator_behavior_)
        {
            using enum OperatorBehavior;
        case CheckedSafe:
        case CheckedSafeMaximalizeAndMinimalizeValue:
            if (position_ != SIZE_MAX) [[likely]]
            {
                position_++;
                return true;
            }

            return false;
        case UncheckedUnsafe:
            position_++;
            return true;
        default:
            return false;
        }
    }

    bool Position::increment_by(const std::size_t amount)
    {
        if (SIZE_MAX - position_ < amount) [[likely]]
        {
            position_ += amount;
            return true;
        }

        if (operator_behavior_ == OperatorBehavior::CheckedSafeMaximalizeAndMinimalizeValue)
        {
            position_ = SIZE_MAX;
        }

        return false;
    }

    bool Position::increment_checked()
    {
        if (position_ == SIZE_MAX) [[unlikely]]
        {
            return false;
        }

        position_++;
        return true;
    }

    bool Position::increment_by_checked(const std::size_t amount)
    {
        if (SIZE_MAX - position_ < amount)
        {
            return false;
        }

        position_ += amount;
        return true;
    }

    bool Position::increment_and_maximalize()
    {
        return increment_checked();
    }

    bool Position::increment_by_and_maximalize(const std::size_t amount)
    {
        if (SIZE_MAX - position_ < amount)
        {
            position_ = SIZE_MAX;
            return false;
        }

        position_ += amount;
        return true;
    }

    void Position::unsafe_unchecked_increment()
    {
        position_++;
    }

    void Position::unsafe_unchecked_increment_by(const std::size_t amount)
    {
        position_ += amount;
    }

    bool Position::decrement()
    {
        switch (operator_behavior_)
        {
            using enum OperatorBehavior;
        case CheckedSafe:
        case CheckedSafeMaximalizeAndMinimalizeValue:
            if (position_ != 0) [[likely]]
            {
                position_--;
                return true;
            }

            return false;
        case UncheckedUnsafe:
            position_--;
            return true;
        default:
            return false;
        }
    }

    bool Position::decrement_by(const std::size_t amount)
    {
        if (amount <= position_)
        {
            position_ -= amount;
            return true;
        }

        if (operator_behavior_ == OperatorBehavior::CheckedSafeMaximalizeAndMinimalizeValue)
        {
            position_ = 0;
        }

        return false;
    }

    bool Position::decrement_checked()
    {
        if (position_ == 0) [[unlikely]]
        {
            return false;
        }

        position_ -= 1;
        return true;
    }

    bool Position::decrement_by_checked(const std::size_t amount)
    {
        if (amount > position_)
        {
            return false;
        }

        position_ -= amount;
        return true;
    }

    bool Position::decrement_and_minimalize()
    {
        return decrement_checked();
    }

    bool Position::decrement_by_and_minimalize(const std::size_t amount)
    {
        if (amount > position_)
        {
            position_ = 0;
            return false;
        }

        position_ -= amount;
        return true;
    }

    void Position::unsafe_unchecked_decrement()
    {
        position_--;
    }

    void Position::unsafe_unchecked_decrement_by(const std::size_t amount)
    {
        position_ -= amount;
    }

    Position &Position::operator++()
    {
        increment();
        return *this;
    }

    Position &Position::operator++(const int amount)
    {
        if (operator_behavior_ == OperatorBehavior::UncheckedUnsafe || amount == 0)
        {
            position_ += amount;
            return *this;
        }

        if (amount < 0)
        {
            const auto size_t_amount = static_cast<std::size_t>(-amount);
            return this->operator-=(size_t_amount);
        }

        const auto size_t_amount = static_cast<std::size_t>(amount);
        return this->operator+=(size_t_amount);
    }

    Position &Position::operator--()
    {
        decrement();
        return *this;
    }

    Position &Position::operator--(const int amount)
    {
        if (operator_behavior_ == OperatorBehavior::UncheckedUnsafe || amount == 0)
        {
            position_ -= amount;
            return *this;
        }

        if (amount < 0)
        {
            const auto size_t_amount = static_cast<std::size_t>(-amount);
            return this->operator+=(size_t_amount);
        }

        const auto size_t_amount = static_cast<std::size_t>(amount);
        return this->operator-=(size_t_amount);
    }

    Position &Position::operator+=(const std::size_t amount)
    {
        increment_by(amount);
        return *this;
    }

    Position &Position::operator-=(const std::size_t amount)
    {
        decrement_by(amount);
        return *this;
    }

    Position::illegal_operator_behavior_exception::illegal_operator_behavior_exception(const OperatorBehavior illegal_operator_behavior)
            : message_(generate_message(illegal_operator_behavior)), illegal_operator_behavior_(illegal_operator_behavior)
    {}

    const std::string &Position::illegal_operator_behavior_exception::message() const
    {
        return message_;
    }

    OperatorBehavior Position::illegal_operator_behavior_exception::illegal_operator_behavior() const
    {
        return illegal_operator_behavior_;
    }

    const char *Position::illegal_operator_behavior_exception::what() const noexcept
    {
        return message_.c_str();
    }

    std::string
    Position::illegal_operator_behavior_exception::generate_message(OperatorBehavior illegal_operator_behavior)
    {
        return "The operator behavior is not a valid operator behavior in the sense that it is not one of the values of the enum OperatorBehavior.\n"
        "Its value is " + std::to_string(static_cast<uint32_t>(illegal_operator_behavior)) + ".\nThe valid integer values are 0, 1 and 2 for CheckedSafe,"
        "CheckedSafeMaximalizeAndMinimalizeValue and UncheckedUnsafe respectively";
    }
} // namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
