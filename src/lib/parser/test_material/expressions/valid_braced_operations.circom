{
    // Syntaxically correct !== Logically correct
    // Only testing the tokens and tree built by the parser. No intelligent checks happen

    func();
    array[123];

    func()[123];
    array[123]();

    func(abc, def[a]);
}