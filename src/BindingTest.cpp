#include <catch2/catch.hpp>

#include "JsonBind.hpp"

struct SimpleStruct {
    int a = -1;
    std::string b = "";
    double c = -1.0;

private:
    friend bool operator==(const SimpleStruct& lhs, const SimpleStruct& rhs) {
        return lhs.a == rhs.a and lhs.b == rhs.b and lhs.c == rhs.c;
    }
};

namespace param {
TEST_CASE("JsonParam") {
    const auto description =
        std::make_tuple(std::make_pair("a", &SimpleStruct::a),
                        std::make_pair("b", &SimpleStruct::b),
                        std::make_pair("c", &SimpleStruct::c));
    SECTION("Empty") {
        const auto output = readJson<SimpleStruct>("{}", description);
        REQUIRE(output == SimpleStruct{});
    }

    SECTION("1 element") {
        const auto output = readJson<SimpleStruct>(
            "{\"a\":1,\"b\":\"2\", \"c\": 3.0}", description);
        REQUIRE(output == SimpleStruct{1, "2", 3.0});
    }
}
}  // namespace param

namespace descriptor {

template <>
struct Descriptor<SimpleStruct> {
    auto getDescription() {
        return std::make_tuple(std::make_pair("a", &SimpleStruct::a),
                               std::make_pair("b", &SimpleStruct::b),
                               std::make_pair("c", &SimpleStruct::c));
    }
};

TEST_CASE("Json Description") {
    SECTION("Empty") {
        const auto output = readJson<SimpleStruct>("{}");
        REQUIRE(output == SimpleStruct{});
    }

    SECTION("1 element") {
        const auto output =
            readJson<SimpleStruct>("{\"a\":1,\"b\":\"2\", \"c\": 3.0}");
        REQUIRE(output == SimpleStruct{1, "2", 3.0});
    }
}
}  // namespace descriptor

namespace compile_description_int {
/*
template <>
struct Descriptor<SimpleStruct> {
    auto getDescription() {
        return std::make_tuple(std::make_pair("a", &SimpleStruct::a),
                               std::make_pair("b", &SimpleStruct::b),
                               std::make_pair("c", &SimpleStruct::c));
    }
};
    SECTION("Empty") {
        const auto output = readJson<SimpleStruct>("{}");
        REQUIRE(output == SimpleStruct{});
    }

    SECTION("1 element") {
        const auto output =
            readJson<SimpleStruct>("{\"a\":1,\"b\":\"2\", \"c\": 3.0}");
        REQUIRE(output == SimpleStruct{1, "2", 3.0});
    }*/

}
TEST_CASE("Data pointer") {
    const auto a_tuple = std::make_tuple("a", &SimpleStruct::a);
    auto SimpleStruct::*ptr = std::get<1>(a_tuple);
    SimpleStruct data;
    data.*ptr = 3;
    REQUIRE(data.a == 3);
}

namespace get_arg {

template <int T, class B>
struct GetArgs {};

template <int index, class R, class... Args>
struct GetArgs<index, R(Args...)> {
    using type = std::tuple_element_t<index, std::tuple<Args...>>;
};

void* testFunc(double filename, int mode);
/* using x = GetArgs<0, decltype(testFunc)>::type::nothing; */

static_assert(std::is_same_v<GetArgs<0, decltype(testFunc)>::type, double>);
static_assert(std::is_same_v<GetArgs<1, decltype(testFunc)>::type, int>);

}  // namespace get_arg

TEST_CASE("Json int key ") {
    // cannot use int as key
    using namespace nlohmann;
    REQUIRE_THROWS(json::parse("{1:1, 2:2}"));
}
