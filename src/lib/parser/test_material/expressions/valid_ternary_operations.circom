{
    // Syntaxically correct !== Logically correct
    // Only testing the tokens and tree built by the parser. No intelligent checks happen

    a ? b : c;
    a ? (b ? c : d) : e;
    a
        ? b
        : c;
}