#include <map>
#include <nlohmann/json.hpp>
#include <string_view>
#include <tuple>

namespace baseline {

template <typename ReturnType>
ReturnType readJson(std::string_view jsonString) {
    using namespace nlohmann;
    ReturnType ret;
    auto j = json::parse(jsonString);
    if (j.empty()) {
        return ret;
    }
    ret.a = j["a"];
    ret.b = j["b"];
    ret.c = j["c"];
    return ret;
}

}  // namespace baseline

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

/*=============================================================
 *=============================================================
 *=============================================================
 */
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

    auto readValue = [&](const auto& key) { return j[key]; };

    auto populateObject = [&](const auto& elemDesc) {
        auto ReturnType::*ptr = std::get<1>(elemDesc);
        ret.*ptr = readValue(std::get<0>(elemDesc));
    };
    const auto description = Descriptor<ReturnType>{}.getDescription();

    std::apply([&](auto&... x) { (..., populateObject(x)); }, description);
    return ret;
}
}  // namespace descriptor

/*=============================================================
 *=============================================================
 *=============================================================
 */

namespace compile_description {
// Create a tuple type that contains the keys and a lambda to populate. Get the
// type of the
// member with GetArgs

template <typename ReturnType>
struct Descriptor {
    using type = std::tuple<ReturnType>;
};

template <auto Value>
struct PtrMemberExtractor {};

template <typename Class, typename Result, Result Class::*Value>
struct PtrMemberExtractor<Value> {
    // add members using Class, Result, and value here
    using containing_type = Class;
    using result = Result;
    static const auto value = Value;
};

template <auto PtrToMember>
constexpr auto setter() {
    using Def = PtrMemberExtractor<PtrToMember>;
    constexpr auto setterf =
        [](Def::containing_type & obj, const Def::result& value) constexpr {
        obj.*(Def::value) = value;
    };
    return setterf;
}

#define SETTER(KEY, PTR) \
    std::pair<decltype([]() { return KEY; }), decltype(setter<PTR>())>

template <typename ReturnType>
ReturnType readJson(std::string_view jsonString) {
    using namespace nlohmann;
    ReturnType ret;
    const auto j = json::parse(jsonString);
    if (j.empty()) {
        return ret;
    }

    auto populateObject = [&](const auto& elemDesc) {
        auto setter = std::get<1>(elemDesc);
        setter(ret, j[std::get<0>(elemDesc)()]);
    };
    const typename Descriptor<ReturnType>::type desc;
    std::apply([&](auto&... x) { (..., populateObject(x)); }, desc);
    return ret;
}

}  // namespace compile_description
