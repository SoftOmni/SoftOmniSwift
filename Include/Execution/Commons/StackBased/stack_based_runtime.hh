
#ifndef SOFTOMNILIB_STACK_BASED_RUNTIME_HH
#define SOFTOMNILIB_STACK_BASED_RUNTIME_HH

#include <Include/Execution/Commons/language_runtime.hh>
#include <Include/Parsing/Commons/Kernels/StackBased/stack_based_frontend.hh>

namespace SoftOmni::Execution::Commons::StackBased
{
    template <typename TPrimitive>
    class StackBasedRuntime: public LanguageRuntime
    {
        using Frontend = Parsing::Commons::Kernels::StackBased::StackBasedLanguageFrontend<TPrimitive>;

        std::size_t stack_evaluation_position_;
    protected:
        explicit StackBasedRuntime(const std::string &name, Frontend &frontend);

        explicit StackBasedRuntime(std::string name, std::string display_name, Frontend &frontend);

    public:
        [[nodiscard]] std::size_t stack_evaluator_position() const;

        virtual void evaluate_without_consuming(int primitive_count, std::optional<std::size_t> new_position_to_adopt = std::nullopt) = 0;

        void evaluate_without_consuming(int primitive_count, bool without_repositioning_stack_evaluator = false);

        void evaluate_without_consuming();

        void relocate_stack_evaluator_position(std::size_t new_position);

        virtual void evaluate_while_consuming(int primitive_count) = 0;

        void evaluate_while_consuming();
    };
}

#endif //SOFTOMNILIB_STACK_BASED_RUNTIME_HH
