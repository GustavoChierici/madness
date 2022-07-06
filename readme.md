# Madness 

- almost functional
- rule-based
- lazy evaluation (short circuiting)

```
let mut x = 5
let mut y = 10

# Doesn't execute
if x == 10 and y == 15 then
    print("hello")
end

when x == 10 and y == 15 do
    print("world")
end

y = 10 + 5 # doesn't notify
x = 10 # notify and rule is executed

```