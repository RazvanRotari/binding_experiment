#define CATCH_CONFIG_ENABLE_BENCHMARKING
#include <catch2/catch.hpp>

#include "JsonBind.hpp"
#include "SimpleStruct.hpp"

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

TEST_CASE("JsonParam benchmark") {
    const auto description =
        std::make_tuple(std::make_pair("a", &SimpleStruct::a),
                        std::make_pair("b", &SimpleStruct::b),
                        std::make_pair("c", &SimpleStruct::c));
    BENCHMARK("Empty") { return readJson<SimpleStruct>("{}", description); };

    BENCHMARK("1 element") {
        return readJson<SimpleStruct>("{\"a\":1,\"b\":\"2\", \"c\": 3.0}",
                                      description);
    };
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

TEST_CASE("Json Description benchmark") {
    BENCHMARK("Empty") { return readJson<SimpleStruct>("{}"); };

    BENCHMARK("1 element") {
        return readJson<SimpleStruct>("{\"a\":1,\"b\":\"2\", \"c\": 3.0}");
    };
}
}  // namespace descriptor

namespace compile_description {

template <>
struct Descriptor<SimpleStruct> {
    using type =
        std::tuple<SETTER("a", &SimpleStruct::a), SETTER("b", &SimpleStruct::b),
                   SETTER("c", &SimpleStruct::c)>;
};

TEST_CASE("JsonCompileParam") {
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
TEST_CASE("JsonCompileParam benchmark") {
    BENCHMARK("Empty") { return readJson<SimpleStruct>("{}"); };

    BENCHMARK("1 element") {
        return readJson<SimpleStruct>("{\"a\":1,\"b\":\"2\", \"c\": 3.0}");
    };
}
}  // namespace compile_description
