{
    // Syntaxically correct !== Logically correct
    // Only testing the tokens and tree built by the parser. No intelligent checks happen

    -a;
    -(a);
    +a;
    +(a);
    !!a;
    + + a;

    ~a + !a;
    (++a - (--a) + +a);
}