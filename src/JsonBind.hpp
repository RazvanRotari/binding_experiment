#include <map>
#include <nlohmann/json.hpp>
#include <string_view>
#include <tuple>
// Get the description as a parameter
namespace param {
template <typename ReturnType, typename... Tuple>
ReturnType readJson(std::string_view jsonString,
                    const std::tuple<Tuple...>& description) {
    using namespace nlohmann;
    ReturnType ret;
    auto j = json::parse(jsonString);
    if (j.empty()) {
        return ret;
    }

    auto readValue = [&](const auto& key) { return j[key]; };

    auto populateObject = [&](const auto& elemDesc) {
        auto ReturnType::*ptr = std::get<1>(elemDesc);
        ret.*ptr = readValue(std::get<0>(elemDesc));
    };

    std::apply([&](auto&... x) { (..., populateObject(x)); }, description);
    return ret;
}
}  // namespace param

namespace descriptor {
struct FakeType {};

template <typename ReturnType>
struct Descriptor {
    auto getDescription() { return FakeType{}; }
};

template <typename ReturnType>
ReturnType readJson(std::string_view jsonString) {
    using namespace nlohmann;
    ReturnType ret;
    auto j = json::parse(jsonString);
    if (j.empty()) {
        return ret;
    }

    auto populateObject = [&](const auto& elemDesc) {
        auto ReturnType::*ptr = std::get<1>(elemDesc);
        ret.*ptr = j[std::get<0>(elemDesc)];
    };
	const auto description = Descriptor<ReturnType>{}.getDescription();

    std::apply([&](auto&... x) { (..., populateObject(x)); },
               description);
    return ret;
}
}  // namespace descriptor
namespace compile_description_int {

template <int T, class B>
struct GetArgs {};

template <int index, class R, class... Args>
struct GetArgs<index, R(Args...)> {
    using type = std::tuple_element_t<index, std::tuple<Args...>>;
};
//Create a tuple type that contains the keys and a lambda to populate. Get the type of the 
// member with GetArgs


}
