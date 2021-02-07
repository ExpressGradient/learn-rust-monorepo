# Fearless Concurrency
Concurrent programming, where different parts of a program execute independently, and parallel programming, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.  

# Using Threads to Run Code Simultaneously
In most current operating systems, an executed program's code is run in a process, and the operating system manages multiple processes at once.  
Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.  
Splitting the computation in your program into multiple threads can improve performance because the program does multiple tasks at the same time, but it also adds complexity.  
Because threads can run simultaneously, there's no inherent guarantee about the order in which parts of your code on different threads will run.  
Programming languages implement threads in a few different ways. Many operating systems provide an API for creating new threads. This model where a language calls the operating system APIs to create threads is sometimes called 1:1, meaning one operating system thread per one language thread.  
Programming language-provided threads are called green threads, and languages that use these green threads will execute them in the context of a different number of operating system threads. For this reason, the green threaded model is called the M:N model: there are M green threads per N operating system threads.  
Each model has its own advantages and trade-offs, and the trade-off most important to Rust is runtime support. Runtime in this context means the code that is included by the language in every binary.  
Smaller runtimes have fewer features but have the advantage of resulting in smaller binaries, which make it easier to combine the language with other languages in more contexts.  
The green threading model requires a larger runtime to manage threads. As such. the Rust standard library only provides an implementation of 1:1 threading.  

## Creating a New Thread with `thread::spawn`
To create a new thread, we call the `thread::spawn` function and pass it a closure containing the code we want to run in a new thread.  
The new thread will be stopped when the main thread ends, whether it has finished running or not.  
The main thread executes first, even though the new thread code appears first in the code.  

## Waiting for All Threads to Finish Using `join` Handles
We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn` in a variable.  
The return type of `thread:spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call `join` method on it will wait for its thread to finish.  
Calling `join` on the handle blocks the thread currently until the thread represented by the handle terminates. Blocking a thread means that thread is prevented from performing work or exiting.  
The two threads continue alternating, but the main thread waits because of the call to `handle.join()` and does not end until the spawned thread is finished.  
If we call `handle.join()` before the main thread, the main thread will wait for the spawned threads to finish and then execute the main thread.  

## Use `move` Closures with Threads
The `move` closure is often used alongside `thread::spawn` because it allows you to use data from one thread in another thread.  
We can use the `move` keyword before the parameter list of a closure to force the closure to take ownership of the values it uses in the environment.