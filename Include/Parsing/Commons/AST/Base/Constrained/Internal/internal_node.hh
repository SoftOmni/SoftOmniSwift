
#ifndef SOFTOMNILIB_INTERNAL_NODE_HH
#define SOFTOMNILIB_INTERNAL_NODE_HH

#include <Include/Parsing/Commons/AST/Base/Constrained/Base/node.hh>
#include <concepts>

#include <Include/Parsing/Commons/AST/Base/Constrained/Internal/internal_node_iterator.hh>

namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Internal
{


    template <typename T, typename ElementType>
    concept iterable = std::ranges::range<T> &&
        std::same_as<std::ranges::range_value_t<T>, ElementType>;

    template <typename Action, typename NodeType>
    concept const_node_action = std::invocable<Action, const NodeType&>;

    template <typename Action, typename NodeType>
    concept mutable_node_action = std::invocable<Action, NodeType&>;

    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename TCharType>
    class InternalNode: public Base::Node<TDerived, TTreeGrouping, TFrontend, TCharType>
    {
        using TBaseNode = Base::Node<TDerived, TTreeGrouping, TFrontend, TCharType>;

    public:
        using immediate_child_iterator = ImmediateChildSentinel<TBaseNode, false>;
        using const_immediate_child_iterator = ImmediateChildSentinel<TBaseNode, true>;

        using child_iterator = ChildSentinel<TBaseNode, false>;
        using const_child_iterator = ChildSentinel<TBaseNode, true>;

    protected:
        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer);

        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            TTreeGrouping::Internal& parent, ParentOffsets::ParentOffset parent_offset);

        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            std::vector<TBaseNode> children);

        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            const iterable<TBaseNode> auto& children);

        template <typename ...Args> requires (std::convertible_to<Args, TBaseNode> && ...)
        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer, Args&&... children);

        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            TTreeGrouping::Internal& parent, ParentOffsets::ParentOffset parent_offset,
            std::vector<TBaseNode> children);

        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            TTreeGrouping::Internal& parent, ParentOffsets::ParentOffset parent_offset,
            const iterable<TBaseNode> auto& children);

        template <typename ...Args> requires (std::convertible_to<Args, TBaseNode> && ...)
        explicit InternalNode(TFrontend& frontend, Buffers::Buffer<TCharType>& buffer,
            TTreeGrouping::Internal& parent, ParentOffsets::ParentOffset parent_offset,
            Args&&... children);

    private:
        std::vector<TBaseNode> nodes_;

    protected:
        const std::vector<TBaseNode>& nodes() const;

    public:
        [[nodiscard]] std::size_t number_of_children() const;

        [[nodiscard]] std::size_t children_capacity() const;

        [[nodiscard]] const TBaseNode &operator[](std::size_t index) const;

        [[nodiscard]] std::vector<TBaseNode>::const_iterator cbegin() const;

        [[nodiscard]] std::vector<TBaseNode>::iterator begin() const;

        [[nodiscard]] const_immediate_child_iterator cbegin(std::predicate<const TBaseNode&> auto predicate) const;

        [[nodiscard]] immediate_child_iterator begin(std::predicate<const TBaseNode&> auto predicate);

        [[nodiscard]] const_immediate_child_iterator cbegin(std::predicate<std::size_t> auto predicate) const;

        [[nodiscard]] immediate_child_iterator begin(std::predicate<std::size_t> auto predicate);

        [[nodiscard]] const_immediate_child_iterator cbegin(std::predicate<std::size_t, const TBaseNode&> auto predicate) const;

        [[nodiscard]] immediate_child_iterator begin(std::predicate<std::size_t, const TBaseNode&> auto predicate);

        [[nodiscard]] const_immediate_child_iterator cend(std::predicate<const TBaseNode&> auto predicate) const;

        [[nodiscard]] immediate_child_iterator end(std::predicate<const TBaseNode&> auto predicate);

        [[nodiscard]] const_immediate_child_iterator cend(std::predicate<std::size_t> auto predicate) const;

        [[nodiscard]] immediate_child_iterator end(std::predicate<std::size_t> auto predicate);

        [[nodiscard]] const_immediate_child_iterator cend(std::predicate<std::size_t, const TBaseNode&> auto predicate) const;

        [[nodiscard]] immediate_child_iterator end(std::predicate<std::size_t, const TBaseNode&> auto predicate);


        [[nodiscard]] const_child_iterator cbegin(Traversal traversal) const;

        [[nodiscard]] child_iterator begin(Traversal traversal);

        [[nodiscard]] const_child_iterator cend(Traversal traversal) const;

        [[nodiscard]] child_iterator end(Traversal traversal);



        [[nodiscard]] const_child_iterator cbegin(Traversal traversal, std::predicate<const TBaseNode&> auto predicate) const;

        [[nodiscard]] child_iterator begin(Traversal traversal, std::predicate<const TBaseNode&> auto predicate);

        [[nodiscard]] const_child_iterator cend(Traversal traversal, std::predicate<const TBaseNode&> auto predicate) const;

        [[nodiscard]] child_iterator end(Traversal traversal, std::predicate<const TBaseNode&> auto predicate);

        [[nodiscard]] const_child_iterator cbegin(Traversal traversal, std::predicate<std::size_t> auto predicate) const;

        [[nodiscard]] child_iterator begin(Traversal traversal, std::predicate<std::size_t> auto predicate);

        [[nodiscard]] const_child_iterator cend(Traversal traversal, std::predicate<std::size_t> auto predicate) const;

        [[nodiscard]] child_iterator end(Traversal traversal, std::predicate<std::size_t> auto predicate);

        [[nodiscard]] const_child_iterator cbegin(Traversal traversal, std::predicate<std::size_t, const TBaseNode&> auto predicate) const;

        [[nodiscard]] child_iterator begin(Traversal traversal, std::predicate<std::size_t, const TBaseNode&> auto predicate);

        [[nodiscard]] const_child_iterator cend(Traversal traversal, std::predicate<std::size_t, const TBaseNode&> auto predicate) const;

        [[nodiscard]] child_iterator end(Traversal traversal, std::predicate<std::size_t, const TBaseNode&> auto predicate);

        [[nodiscard]] std::vector<TBaseNode>::const_iterator cend() const;

        [[nodiscard]] std::vector<TBaseNode>::iterator end() const;

        [[nodiscard]] bool contains_immediate_child(const TBaseNode& child) const;

        [[nodiscard]] bool contains_all_as_immediate_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_all_as_immediate_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode&> && ...)
        [[nodiscard]] bool contains_all_as_immediate_children(Args&&... children) const;

        [[nodiscard]] bool contains_some_as_immediate_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_some_as_immediate_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode&> && ...)
        [[nodiscard]] bool contains_some_as_immediate_children(Args&&... children) const;

        [[nodiscard]] bool contains_none_as_immediate_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_none_as_immediate_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode&> && ...)
        [[nodiscard]] bool contains_none_as_immediate_children(Args&&... children) const;

        [[nodiscard]] bool contains_any_as_immediate_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_any_as_immediate_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode &> && ...)
        [[nodiscard]] bool contains_any_as_immediate_children(Args &&...children) const;

        [[nodiscard]] bool contains_child(const TBaseNode& child) const;

        [[nodiscard]] bool contains_all_as_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_all_as_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode&> && ...)
        [[nodiscard]] bool contains_all_as_children(Args&&... children) const;

        [[nodiscard]] bool contains_some_as_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_some_as_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode&> && ...)
        [[nodiscard]] bool contains_some_as_children(Args&&... children) const;

        [[nodiscard]] bool contains_none_as_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_none_as_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode &> && ...)
        [[nodiscard]] bool contains_none_as_children(Args &&...children) const;

        [[nodiscard]] bool contains_any_as_children(const std::vector<const TBaseNode&>& children) const;

        [[nodiscard]] bool contains_any_as_children(const iterable<const TBaseNode&> auto& children) const;

        template <typename... Args>
            requires(std::convertible_to<Args, const TBaseNode &> && ...)
        [[nodiscard]] bool contains_any_as_children(Args &&...children) const;

        [[nodiscard]] std::optional<const TBaseNode&> find_first_where(std::predicate<const TBaseNode&> auto predicate, Traversal traversal = Traversal::LINEAR_NO_RECURSE_INTO_CHILDREN) const;

        [[nodiscard]] std::optional<const TBaseNode&> find_first_where_not(std::predicate<const TBaseNode&> auto predicate, Traversal traversal = Traversal::LINEAR_NO_RECURSE_INTO_CHILDREN) const;

        [[nodiscard]] std::optional<const TBaseNode&> find_last_where(std::predicate<const TBaseNode&> auto predicate, Traversal traversal = Traversal::LINEAR_NO_RECURSE_INTO_CHILDREN) const;

        [[nodiscard]] std::optional<const TBaseNode&> find_last_where_not(std::predicate<const TBaseNode&> auto predicate, Traversal traversal = Traversal::LINEAR_NO_RECURSE_INTO_CHILDREN) const;

        [[nodiscard]] std::vector<std::reference_wrapper<const TBaseNode>> find_all_where(std::predicate<const TBaseNode&> auto predicate, Traversal traversal = Traversal::LINEAR_NO_RECURSE_INTO_CHILDREN) const;

        [[nodiscard]] std::vector<std::reference_wrapper<const TBaseNode>> find_all_where_not(std::predicate<const TBaseNode&> auto predicate, Traversal traversal = Traversal::LINEAR_NO_RECURSE_INTO_CHILDREN) const;

        template <const_node_action<TBaseNode> Action>
        void traverse(Action action, Traversal traversal) const;

        template <mutable_node_action<TBaseNode> Action>
        void traverse_and_modify(Action action, Traversal traversal);

        bool prepend_child_at(std::unique_ptr<TBaseNode> new_child);

        std::size_t prepend_children_at(const std::vector<std::unique_ptr<TBaseNode>> &new_children, bool reverse = false);

        std::size_t prepend_children_at(const iterable<std::unique_ptr<TBaseNode>> auto &new_children, bool reverse = false);

        template <typename... Args>
            requires(std::convertible_to<Args, std::unique_ptr<TBaseNode>> && ...)
        std::size_t prepend_children_at(Args &&...new_children, bool reverse = false);

        bool append_child_at(std::unique_ptr<TBaseNode> new_child);

        std::size_t append_children_at(const iterable<std::unique_ptr<TBaseNode>> auto &new_children,
                                       bool reverse = false);

        std::size_t append_children_at(const std::vector<std::unique_ptr<TBaseNode>> &new_children,
                                       bool reverse = false);
        template <typename... Args>
            requires(std::convertible_to<Args, TBaseNode> && ...)
        std::size_t append_children_at(Args &&...new_children, bool reverse = false);

        bool insert_child_at(std::size_t index, std::unique_ptr<TBaseNode> new_child);

        std::size_t insert_children_at(std::size_t index, const std::vector<std::unique_ptr<TBaseNode>>& new_children, bool reverse = false);

        std::size_t insert_children_at(std::size_t index, const iterable<std::unique_ptr<TBaseNode>> auto& new_children, bool reverse = false);

        template <typename ...Args> requires (std::convertible_to<Args, std::unique_ptr<TBaseNode>> && ...)
        std::size_t insert_children_at(std::size_t index, Args&&... new_children, bool reverse = false);
    };

} // namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Internal

#endif //SOFTOMNILIB_INTERNAL_NODE_HH
