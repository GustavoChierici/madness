# Madness

#### An almost functional, rule-based and lazy evaluated language

## Milestones

- [] Make a simple interpreter for the language in Rust 
- [] Make it Turing Complete (Rule 110 algorithm)   
- [] Make a LLVM front-end for the language in C++ (to be decided)
- [] Make a compiler for the language in itself (bootstrapping)
- [] Implement new features
- [] Make an IoT application

```
x: int = 5
y: int = 10

// Doesn't execute
if x == 10 and y == 15 {
    print("hello")
}


when x == 10 and y == 15 {
    print("world")
}

y = 10 + 5 // doesn't notify
x = 10 // notify and rule is executed
```