{
    // Syntaxically correct !== Logically correct
    // Only testing the tokens and tree built by the parser. No intelligent checks happen

    a * b;
    a / b;
    a % b;
    a \ b;

    a * b \ (c % (d / 4));
}