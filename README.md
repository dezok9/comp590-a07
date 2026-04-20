# Assignent #7

## Team Members
- Destiny Okonkwo
- Jimena Rivera
- Grace Odondi

## Files
**`main.rs`** => Contains the implemenation for `collatz_length()` and `longest_collatz()`.

## How to Compile & Run
**Step #1:** Open a terminal

**Step #2:** Compile the Rust file, `rustc main.rs`

**Step #3:** Run, `./main`

## Discussion Question
**Your `collatz_length` function calls itself potentially hundreds of times for a single input. What happens to the call stack each time it recurses? Is there a risk here that would not exist if you had used a loop instead? Does Rust do anything to help with this?**
Each time `collatz_length` recurses, the call stack adds a new entry onto the stack, taking up memory and increasing the stack depth. As it is _not_ tail recursive (a recursive call is **not** its last operation; it adds 1 as its last operation) each time, this makes `collatz_length` not space effient and means that it may take hundreds of frames to run. Because of this, there the risk of the program running out of memory and the program overflowing its stack. This is in contrast to if we used a loop--with a loop, memory is freed as we leave the scope meaning that we would not be increasing stack depth in the matter and to the extent that we do with recursion. Currently, Rust does not do much to help with this when using recursion other than producing an output indicating that the stack has overflowed at runtime. Additionally, Rust does not guarantee that it will optimize code into a loop like Erlang or Haskell (aka does not guarantee tail recursion optimization).
