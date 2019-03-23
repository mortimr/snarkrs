{
    // Syntaxically correct !== Logically correct
    // Only testing the tokens and tree built by the parser. No intelligent checks happen

    a === b;
    (a + b) === (c - d);
    a === (b + c);
    (b + c) === a;
    (b === c) === d;

    a <-- b;
    (a + b) <-- (c - d);
    a <-- (b + c);
    (b + c) <-- a;
    (b <-- c) <-- d;
    
    a <== b;
    (a + b) <== (c - d);
    a <== (b + c);
    (b + c) <== a;
    (b <== c) <== d;
    
    a --> b;
    (a + b) --> (c - d);
    a --> (b + c);
    (b + c) --> a;
    (b --> c) --> d;

    a ==> b;
    (a + b) ==> (c - d);
    a ==> (b + c);
    (b + c) ==> a;
    (b ==> c) ==> d;
}