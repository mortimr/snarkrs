function test(arg1, arg2) {

    // Variable declaration
    var a;
    var b = 1;
    var c = 0x2;
    var d = a;
    var e = func(arg1);

    //// Signal declaration
    signal sig_a;
    signal input sig_b;
    signal output sig_c;
    signal private input sig_d;

    //// Component declaration
    component comp_a = Component();
    component comp_b = Component(arg2);
    component Component() comp_c;
    component Component(arg2) comp_d;

    // Main Expression Types (
    12345;
    0x12345;
    abc;
    abc();
    abc(1);
    abc(abc);
    abc(0x123);
    abc(1, abc, 0x123);
    (abc)(1, abc, 0x123);
    (((abc)))(1, abc, 0x123);
    abc[123];
    abc[123](456);
    abc[123](456)[789](101112);
    a + b * c;
    a * (b + c);

    // Assignment Statement
    a = 2;
    a += 2;
    a -= 2;
    a *= 2;
    a /= 2;
    a %= 2;
    a <<= 2;
    a >>= 2;
    a &= 2;
    a ^= 2;
    a |= 2;

    // Signal Operations
    sig_a <== a;
    sig_b <-- b;
    a ==> sig_a;
    b ==> sig_b;
    sig_c * (sig_c - 1) === 0;

    // Logical Operations
    a && b;
    a && b || c;
    a || b && c;
    c || b;

    // Bitwise Operations
    a | b;
    a ^ b;
    a & b;
    a | b & c;
    a | b & c ^ d;

    // Equality Operations
    a == b;
    a != b;
    a * b == c;
    a == b * c;
    a != b * c;

    // Ordering Operations
    a < b;
    a <= b;
    a > b;
    a >= b;
    a * b > c * d;

    // Bitwise Shift Operations
    a >> b;
    a << b;
    a >> (b + 2);
    (a + 2) >> (b + 2);

    // Sum Operations
    a + b;
    a - b;
    a + b - (a + c);

    // Product Operations
    a * b;
    a / b;
    a % b;
    a \ b;
    a \ [1, 2, 3];

    // Exponential Operations
    a ** b;
    a ** (b + 2);

    // Prefix Operations
    !a;
    ~a;
    ++a;
    --a;
    +a;
    -a;

    // Postfix Operations
    a++;
    b--;
    (a++)--;

    // Braced Operations
    a();
    a[1];
    a()(a)[a]()[1];

    // If Blocks

    if (a < b)
        a = b + 1;

    if (a < b) {
        a = b;
        ++a;
    }

    if (a < b ) {
        a = b;
    } else {
        b = a;
    }

    if (a == 1) {
        return 1;
    } else if (a == 2) {
        a = b;
    } else {
        b = a;
    }

    if (a == 1) {

        if (b == 1) {
            return 5;
        }

        while (b == 1) {
            return ;
        }

        for (var idx = 0; idx < 5; ++idx) {
            return ;
        }

        do {
            return ;
        } while (idx < 5)

    }

    // While Blocks

    while (a < b)
        ++a;

    while (a < b) {
        a += 1;
    }

    while (a < b) {
        if (a == 2) break;
        ++a;
    }

    while (a < b) {

        if (b == 1) {
            return 5;
        }

        while (b == 1) {
            return ;
        }

        for (var idx = 0; idx < 5; ++idx) {
            return ;
        }

        do {
            return ;
        } while (idx < 5)

        ++a;

    }

    // For Blocks

    for (var idx = 0; idx < b; ++idx);

    for (var idx = 0; idx < b; ++idx)
        print(123);

    for (var idx = 0; idx < b; ++idx) {
        print(123);
    }

    for (;;) {
        print(123);
    }

    for (var a;;) {
        print(123);
    }

    for (;a = 2;) {
        print(123);
    }

    for (;;++a) {
        print(123);
    }

    for (var idx = 0; idx < b; ++idx) {

        if (b == 1) {
            return 5;
        }

        while (b == 1) {
            return ;
        }

        for (var idx = 0; idx < 5; ++idx) {
            return ;
        }

        do {
            return ;
        } while (idx < 5)

    }

    // Do While Statements

    do
        ++a;
    while (a < b)

    do {
        a += 1;
    } while (a < b)

    do {
        if (a == 2) break;
        ++a;
    } while (a < b)

    do {

        if (b == 1) {
            return 5;
        }

        while (b == 1) {
            return ;
        }

        for (var idx = 0; idx < 5; ++idx) {
            return ;
        }

        do {
            return ;
        } while (idx < 5)

        ++a;

    } while (a < b)

    // SubScope Statement

    {
        var a;
    }

    {
        do {

            if (b == 1) {
                return 5;
            }

            while (b == 1) {
                return ;
            }

            {
                {
                    var deep;
                }
            }

            for (var idx = 0; idx < 5; ++idx) {
                return ;
            }

            do {
                return ;
            } while (idx < 5)

            ++a;

        } while (a < b)
    }
}
