#pragma once
struct SimpleStruct {
    int a = -1;
    std::string b = "";
    double c = -1.0;

private:
    friend bool operator==(const SimpleStruct& lhs, const SimpleStruct& rhs) {
        return lhs.a == rhs.a and lhs.b == rhs.b and lhs.c == rhs.c;
    }
};
