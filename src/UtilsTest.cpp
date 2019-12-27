#include <catch2/catch.hpp>
#include <map>
#include <nlohmann/json.hpp>
#include <string_view>
#include <tuple>

#include "SimpleStruct.hpp"

namespace get_arg {

template <int T, class B>
struct GetArgs {};

template <int index, class R, class... Args>
struct GetArgs<index, R(Args...)> {
    using type = std::tuple_element_t<index, std::tuple<Args...>>;
};

void* testFunc(double filename, int mode);

static_assert(std::is_same_v<GetArgs<0, decltype(testFunc)>::type, double>);
static_assert(std::is_same_v<GetArgs<1, decltype(testFunc)>::type, int>);

}  // namespace get_arg

namespace compile_description_int {
TEST_CASE("Data pointer") {
    const auto a_tuple = std::make_tuple("a", &SimpleStruct::a);
    auto SimpleStruct::*ptr = std::get<1>(a_tuple);
    SimpleStruct data;
    data.*ptr = 3;
    REQUIRE(data.a == 3);
}

}  // namespace compile_description_int

TEST_CASE("Json int key ") {
    // cannot use int as key
    using namespace nlohmann;
    REQUIRE_THROWS(json::parse("{1:1, 2:2}"));
}

namespace compile_description_experiment {
/* constexpr auto l1 = []() { return 5; }; */
TEST_CASE("tuple_experiment") {
    using Desc = std::tuple<decltype([]() { return 5; })>;
    using Func = std::tuple_element_t<0, Desc>;
    REQUIRE(Func()() == 5);
}

template <typename Obj, typename Type>
auto f(Type Obj::*Ptr) {
    /* using x = Ptr::bla; */
    return Ptr;
}

template <auto Value>
struct MyStruct {};

template <typename Class, typename Result, Result Class::*Value>
struct MyStruct<Value> {
    // add members using Class, Result, and value here
    using containing_type = Class;
    using result = Result;
    static const auto value = Value;
};

template <auto A>
auto f2() {
    using Def = MyStruct<A>;
    return [](Def::containing_type& b, const Def::result& a) {
        b.*(Def::value) = a;
    };
}
TEST_CASE("Pointer to member") {
    SimpleStruct s{1, "2", 3.0};
    auto ptr = f(&SimpleStruct::a);
    auto setter = f2<&SimpleStruct::a>();
    setter(s, 3);
    REQUIRE(s.a == 3);
}

}  // namespace compile_description_experiment
