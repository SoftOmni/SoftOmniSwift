
#ifndef SOFTOMNISWIFT_LANGUAGE_HH
#define SOFTOMNISWIFT_LANGUAGE_HH
#include <memory>
#include <optional>
#include <ostream>
#include <unordered_map>

#include <Include/Parsing/Commons/Kernels/language_frontend.hh>

namespace SoftOmni::Parsing::Commons::Kernels
{
    class Language
    {
        std::string name_;

        std::string display_name_;

        std::size_t softomni_format_codepoint_;

        std::size_t softomni_format_codepoint_complement_;

        std::unordered_map<std::string, LanguageFrontend*> language_frontends_;

        std::optional<std::reference_wrapper<LanguageFrontend>> reference_frontend_;

        static std::unordered_map<std::string, Language*> languages_;

    public:
        explicit Language(const std::string& name, std::size_t softomni_format_codepoint, std::size_t softomni_format_codepoint_complement);

        explicit Language(const std::string& name, std::string display_name, std::size_t softomni_format_codepoint, std::size_t softomni_format_codepoint_complement);

        [[nodiscard]] const std::string& name() const;

        [[nodiscard]] const std::string& display_name() const;

        void set_display_name(std::string new_display_name);

        [[nodiscard]] std::size_t softomni_format_codepoint() const;

        [[nodiscard]] std::size_t softomni_format_codepoint_complement() const;

        std::optional<std::reference_wrapper<LanguageFrontend>> operator[](const std::string& name) const;

        static std::optional<std::reference_wrapper<Language>> get(const std::string& name);

        friend bool operator==(const Language& lhs, const Language& rhs)
        {
            return lhs.name_ == rhs.name_;
        }

        bool operator==(const std::string_view& rhs) const;

        friend std::size_t hash_value(const Language& obj)
        {
            return std::hash<std::string>{}(obj.name_);
        }

        friend std::ostream& operator<<(std::ostream& os, const Language& obj)
        {
            os << obj.name_;
            return os;
        }

        friend LanguageFrontend;

        Language(const Language& other) = default;

        Language(Language&& other) noexcept;

        Language& operator=(const Language& other);

        Language& operator=(Language&& other) noexcept;

        ~Language();

        class LanguageAlreadyRegisteredException : public std::exception
        {
            std::string name_;

            std::string message_;

            explicit LanguageAlreadyRegisteredException(std::string name);

        public:
            [[nodiscard]] const std::string& name() const;

            [[nodiscard]] const std::string& message() const;

            [[nodiscard]] const Language& get_already_existing_language() const;

            [[nodiscard]] const char* what() const noexcept override;

            friend class Language;

        private:
            static std::string generate_message(const std::string& name);
        };

    private:
        void setup_language(const std::string& name);

        static LanguageAlreadyRegisteredException
        generate_language_already_registered_exception(const std::string& name);
    };
} // namespace SoftOmni::Parsing::Commons::Kernels

#endif // SOFTOMNISWIFT_LANGUAGE_HH
