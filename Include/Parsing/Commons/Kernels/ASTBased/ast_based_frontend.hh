

#ifndef SOFTOMNILIB_AST_BASED_FRONTEND_HH
#define SOFTOMNILIB_AST_BASED_FRONTEND_HH

#include <Parsing/Commons/Kernels/language_frontend.hh>
#include <map>

#include "Parsing/Commons/AST/Base/Constrained/Root/root_node.hh"
#include "Parsing/Commons/Parsing/parser.hh"

namespace SoftOmni::Parsing::Commons::Kernels::ASTBased
{
    template <typename TDerivedASTFrontend, typename TTreeGroup, typename TCharType = char>
    class ASTBasedFrontend: public LanguageFrontend
    {
        static_assert(std::is_base_of_v<ASTBasedFrontend, TDerivedASTFrontend>, "TDerivedASTFrontend must be derived from ASTBasedFrontend");

    protected:
        std::map<Buffers::Buffer<TCharType>, std::pair<AST::Base::Constrained::Root::RootNode<TTreeGroup, TDerivedASTFrontend, TCharType>, Parsing::Parser<TDerivedASTFrontend>>> buffers_;

        std::vector<std::reference_wrapper<AST::Base::Constrained::Root::RootNode<TTreeGroup, TDerivedASTFrontend, TCharType>>> root_nodes_;

    public:
        explicit ASTBasedFrontend(const std::string &name, const std::string &language);

        explicit ASTBasedFrontend(const std::string &name, Language &language);

        explicit ASTBasedFrontend(std::string name, std::string display_name, const std::string &language);

        explicit ASTBasedFrontend(std::string name, std::string display_name, Language& language);

        [[nodiscard]] const std::vector<const AST::Base::Constrained::Root::RootNode<TTreeGroup, TDerivedASTFrontend, TCharType>>& root_node() const;

        [[nodiscard]] std::size_t number_of_root_nodes() const;

        void add_buffer(Buffers::Buffer<TCharType>, const Parsing::Parser<TDerivedASTFrontend>& parser);


    };
}

#endif // SOFTOMNILIB_AST_BASED_FRONTEND_HH
