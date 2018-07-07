# spinner

This little program can be used to quickly verify that a multithreaded Rust
program uses multiple cores. Pass an integer as arg[1] to specify the number of
threads, then use top to watch CPU usage. Threads just spin in "loop {}", so
watch out for compiler optimizations.
