template test() {
    var a;
    var b_c = 3;
    var c = test_lel(123);

    b_c += 5;
    c >>= b_c;

    c = b_c.member;
    b_c.member = c;

    c = a + b - c * a / d + 6 * (1 + test_lel(abcd + 4)) / 4 % 6;

    a <--- b + c;
    a <=== b + c;
    b + c ---> a;
    b + c ===> a;

    a + c === c;
}