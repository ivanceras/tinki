# Why Rust
 - It's efficient
    - Unlike languages such as Java, Scala and Go, Rust can work out how long memory is used at compile time, and so doesn't need to do runtime garbage collection.
 - It's productive
    - It has all the standard features you'd expect from a modern programming language - things like closures, generics, pattern matching and type inference.
 - It's robust
    - Unlike C and C++, Rust's memory model prevents you from reading or writing to memory after it's been freed.
    - Further, the memory model extends to concurrent operations, meaning that Rust won't allow you to write code that reads and writes to the same memory on different threads, without that being protected by a lock or other synchronization primitive.
   ![](./Linux-CVEs-in-2018.png)

