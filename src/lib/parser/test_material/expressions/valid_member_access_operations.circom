{
    // Syntaxically correct !== Logically correct
    // Only testing the tokens and tree built by the parser. No intelligent checks happen

    a.member;
    (a + b).member;
    a.member.sub_member;
    ++a.member.sub_member;
}