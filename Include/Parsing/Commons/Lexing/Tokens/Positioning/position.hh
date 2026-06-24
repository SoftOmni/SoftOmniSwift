
#ifndef SOFTOMNILIB_POSITION_HH
#define SOFTOMNILIB_POSITION_HH

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>

namespace SoftOmni::Parsing::Commons::Lexing::Tokens::Positioning
{
    enum class OperatorBehavior
    {
        CheckedSafe,
        CheckedSafeMaximalizeAndMinimalizeValue,
        UncheckedUnsafe
    };

    class Position
    {
    protected:
        OperatorBehavior operator_behavior_;

        std::size_t position_;

    public:
        explicit Position(OperatorBehavior operator_behavior = OperatorBehavior::CheckedSafe);

        explicit Position(std::size_t position, OperatorBehavior operator_behavior = OperatorBehavior::CheckedSafe);

        [[nodiscard]] OperatorBehavior operator_behavior() const;

        void set_operator_behavior(OperatorBehavior new_operator_behavior);

        [[nodiscard]] std::size_t position() const;

        void set_position(std::size_t new_position);
        
        bool increment();
        
        bool increment_by(std::size_t amount);
        
        bool increment_checked();
        
        bool increment_by_checked(std::size_t amount);
        
        bool increment_and_maximalize();
        
        bool increment_by_and_maximalize(std::size_t amount);
        
        void unsafe_unchecked_increment();
        
        void unsafe_unchecked_increment_by(std::size_t amount);
        
        bool decrement();
        
        bool decrement_by(std::size_t amount);
        
        bool decrement_checked();
        
        bool decrement_by_checked(std::size_t amount);
        
        bool decrement_and_minimalize();
        
        bool decrement_by_and_minimalize(std::size_t amount);
        
        void unsafe_unchecked_decrement();
        
        void unsafe_unchecked_decrement_by(std::size_t amount);

        Position& operator++();

        Position& operator++(int amount);

        Position& operator--();

        Position& operator--(int amount);

        Position& operator+=(std::size_t amount);

        Position& operator-=(std::size_t amount);

        friend std::optional<Position> operator+(const Position& lhs, const Position& rhs)
        {
            const OperatorBehavior combined_operator_behavior = std::min(lhs.operator_behavior(), rhs.operator_behavior());
            using enum OperatorBehavior;
            if (combined_operator_behavior == UncheckedUnsafe)
            {
                return Position(lhs.position_ + rhs.position_, combined_operator_behavior);
            }
        
            if (SIZE_MAX - lhs.position_ <= rhs.position_)
            {
                return Position(lhs.position_ + rhs.position_, combined_operator_behavior);
            }
        
            if (combined_operator_behavior == CheckedSafeMaximalizeAndMinimalizeValue)
            {
                return Position(SIZE_MAX, combined_operator_behavior);
            }
        
            return std::nullopt;
        }

        friend std::optional<Position> operator-(const Position& lhs, const Position& rhs)
        {
            const OperatorBehavior combined_operator_behavior = std::min(lhs.operator_behavior(), rhs.operator_behavior());
            using enum OperatorBehavior;
            if (combined_operator_behavior == UncheckedUnsafe)
            {
                return Position(lhs.position_ + rhs.position_, combined_operator_behavior);
            }
        
            if (rhs.position_ <= lhs.position_)
            {
                return Position(lhs.position_ + rhs.position_, combined_operator_behavior);
            }
        
            if (combined_operator_behavior == CheckedSafeMaximalizeAndMinimalizeValue)
            {
                return Position(SIZE_MAX, combined_operator_behavior);
            }
        
            return std::nullopt;
        }

        class illegal_operator_behavior_exception: std::exception
        {
            std::string message_;

            OperatorBehavior illegal_operator_behavior_;

            explicit illegal_operator_behavior_exception(OperatorBehavior illegal_operator_behavior);

        public:
            [[nodiscard]] const std::string& message() const;

            [[nodiscard]] OperatorBehavior illegal_operator_behavior() const;

            [[nodiscard]] const char *what() const noexcept override;

            friend Position;

        private:
            static std::string generate_message(OperatorBehavior illegal_operator_behavior);
        };
    };
}

#endif //SOFTOMNILIB_POSITION_HH
