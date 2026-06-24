

#ifndef SOFTOMNILIB_INTERNAL_NODE_ITERATOR_HH
#define SOFTOMNILIB_INTERNAL_NODE_ITERATOR_HH

#include <Include/Parsing/Commons/AST/Base/traversal.hh>
#include <queue>
#include <stack>

namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Internal
{
    template <typename TDerived, typename TTreeGrouping, typename TFrontend, typename TChildType>
    class InternalNode;

    template <typename P, typename NodeType>
    concept indexed_node_predicate = std::predicate<P, std::size_t, const NodeType&>;

    template <typename NodeType, bool IsConst, indexed_node_predicate<NodeType> Predicate>
    class ImmediateChildIterator
    {
    public:
        using node_type = std::conditional_t<IsConst, const NodeType, NodeType>;
        using underlying_iterator = std::conditional_t<
            IsConst,
        typename std::vector<NodeType>::const_iterator,
        typename std::vector<NodeType>::iterator
        >;

        using iterator_category = std::forward_iterator_tag;
        using value_type = node_type;
        using difference_type = std::ptrdiff_t;
        using pointer = node_type*;
        using reference = node_type&;

    private:
        underlying_iterator current_;
        underlying_iterator end_;
        std::size_t index_;
        [[no_unique_address]] Predicate predicate_;

        void advance_to_valid();

    public:
        ImmediateChildIterator() = default;

        ImmediateChildIterator(underlying_iterator begin, underlying_iterator end, Predicate predicate);

        ImmediateChildIterator(underlying_iterator end);

        reference operator*() const;

        pointer operator->() const;

        ImmediateChildIterator& operator++();

        ImmediateChildIterator& operator++(int value);

        bool operator==(const ImmediateChildIterator& other) const;

        bool operator!=(const ImmediateChildIterator& other) const;

        underlying_iterator base() const;

        std::size_t index() const;
    };

    template <typename NodeType, bool IsConst, Traversal TraversalOrder, indexed_node_predicate<NodeType> Predicate>
    class ChildIterator
    {
    public:
        using node_type = std::conditional_t<IsConst, const NodeType, NodeType>;
        using node_pointer = std::conditional_t<IsConst, const NodeType *, NodeType *>;

        using iterator_category = std::forward_iterator_tag;
        using value_type = node_type;
        using difference_type = std::ptrdiff_t;
        using pointer = node_pointer;
        using reference = node_pointer &;

    private:
        struct StackEntry
        {
            node_pointer node;
            std::size_t child_index;
            std::size_t global_index;
        };

        using container_type =
            std::conditional_t<TraversalOrder == Traversal::BREADTH_FIRST_SEARCH ||
                                   TraversalOrder == Traversal::BREADTH_FIRST_SEARCH_LAST_INDEX_COMES_FIRST,
                               std::queue<StackEntry>, std::stack<StackEntry>>;

        container_type pending_;
        node_pointer current_;
        std::size_t global_index_;
        [[no_unique_address]] Predicate predicate_;
        bool at_end_;

        static constexpr bool is_reversed()
        {
            return TraversalOrder == Traversal::DEPTH_FIRST_SEARCH_LAST_INDEX_COMES_FIRST ||
                TraversalOrder == Traversal::BREADTH_FIRST_SEARCH_LAST_INDEX_COMES_FIRST;
        }

        static constexpr bool is_bfs()
        {
            return TraversalOrder == Traversal::BREADTH_FIRST_SEARCH ||
                TraversalOrder == Traversal::BREADTH_FIRST_SEARCH_LAST_INDEX_COMES_FIRST;
        }

        void push_entry(StackEntry entry);

        StackEntry pop_entry();

        bool has_pending() const;

        static bool has_children(node_pointer node);

        static std::size_t get_child_count(node_pointer node);

        static node_pointer get_child(node_pointer node, std::size_t index);

        void enqueue_children(node_pointer node);

        void advance_to_valid();

    public:
        constexpr ChildIterator();

        template <typename Container>
        constexpr ChildIterator(Container &children, Predicate predicate);

        constexpr explicit ChildIterator(std::nullopt_t);

        constexpr reference operator*() const;

        constexpr pointer operator->() const;

        constexpr ChildIterator &operator++();

        constexpr ChildIterator operator++(int offset);

        constexpr bool operator==(const ChildIterator &other) const;

        constexpr bool operator!=(const ChildIterator &other) const;

        constexpr pointer base() const;

        constexpr bool is_end() const;
    };

    template <typename NodeType, bool IsConst>
    class ImmediateChildSentinel
    {
    public:
        using underlying_iterator = std::conditional_t<
            IsConst,
        typename std::vector<NodeType>::const_iterator,
        typename std::vector<NodeType>::iterator>;

    private:
        underlying_iterator end_;

    public:
        constexpr explicit ImmediateChildSentinel(underlying_iterator end) : end_(end) {}

        template <indexed_node_predicate<NodeType> Predicate>
        constexpr bool operator==(const ImmediateChildIterator<NodeType, IsConst, Predicate>& iterator) const
        {
            return iterator.base() == end_;
        }

        template <indexed_node_predicate<NodeType> Predicate>
        friend constexpr bool operator==(
            const ImmediateChildIterator<NodeType, IsConst, Predicate>& iterator,
            const ImmediateChildSentinel& sentinel)
        {
            return iterator.base() == sentinel.end_;
        }
    };

    template <typename NodeType, bool IsConst>
    class ChildSentinel {
    public:
        constexpr ChildSentinel() = default;

        template <Traversal T, indexed_node_predicate<NodeType> Predicate>
        constexpr bool operator==(const ChildIterator<NodeType, IsConst, T, Predicate>& iter) const {
            return iter.is_end();
        }

        template <Traversal T, indexed_node_predicate<NodeType> Predicate>
        friend constexpr bool operator==(
            const ChildIterator<NodeType, IsConst, T, Predicate>& iter,
            const ChildSentinel&) {
            return iter.is_end();
        }
    };

    template <typename NodeType, std::predicate<const NodeType&> Predicate>
    struct NodeOnlyPredicateAdapter
    {
        [[no_unique_address]] Predicate predicate;

        constexpr bool operator()(std::size_t, const NodeType& node) const
        {
            return predicate(node);
        }
    };

    template <std::predicate<std::size_t> Predicate>
    struct IndexOnlyPredicateAdapter
    {
        [[no_unique_address]] Predicate predicate;

        template <typename NodeType>
        constexpr bool operator()(std::size_t index, const NodeType&) const
        {
            return predicate(index);
        }
    };

    struct TruePredicate
    {
        template <typename NodeType>
        constexpr bool operator()(std::size_t, const NodeType&) const
        {
            return true;
        }
    };


    template <typename NodeType, std::predicate<const NodeType&> Predicate>
    constexpr auto adapt_predicate(Predicate&& predicate)
    {
        return NodeOnlyPredicateAdapter<NodeType, std::decay_t<Predicate>>{std::forward<Predicate>(predicate)};
    }

    template <typename NodeType, std::predicate<std::size_t> Predicate>
    constexpr auto adapt_index_predicate(Predicate&& predicate)
    {
        return IndexOnlyPredicateAdapter<std::decay_t<Predicate>>{std::forward<Predicate>(predicate)};
    }

} // namespace SoftOmni::Parsing::Commons::AST::Base::Constrained::Internal


#endif //SOFTOMNILIB_INTERNAL_NODE_ITERATOR_HH
