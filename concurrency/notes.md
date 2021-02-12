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

# Using Message Passing to Transfer Data Between Threads
One popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.  
One major tool Rust has for accomplishing message-sending concurrency is the channel. A channel in programming has two halves: a transmitter and a receiver.  
One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages.  
A channel is said to be closed if either the transmitter or receiver half is dropped.  
We create a channel using `mpsc::channel` function; `mpsc` stands for `multiple producer single consumer`.  
The way Rust's standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.  
The spawned thread needs to own the transmitting end of the channel to be able to send messages through the channel. The transmitting end has a `send` method that takes the value to send.  
The receiving end of the channel has two useful methods: `recv` and `try_recv`. `recv` will block the main thread's execution and wait until a value is sent down the channel.  
When the sending end of the channel closes, `recv` will return an error to signal that no more values will be coming.  
The `try_recv` doesn't block, but it will return a `Result<T, E>` immediately: an `Ok` value holding a message if one is available and an `Err` value if there aren't any messages this time.  

## Channels and Ownership Transference
The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it.  
This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.  

## Creating Multiple Producers
We can create multiple producers by cloning the transmitting half of the channel.  
This time, before we create the first spawned thread, we call `clone` on the sending end of the channel. This will give us a new sending handle we can pass to the first spawned thread.  

# Shared-State Concurrency
Shared memory Concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.  

## Using Mutexes to Allow Access to Data from One Thread at a Time
Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.  
To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex's lock.  
The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.  
You must attempt to acquire the lock before using the data.  
When you're doing with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.  
Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels.  

## The API of `Mutex<T>`
As with many types, we create a `Mutex<T>` using the associated function `new`.  
To access the data inside the mutex, we use the `lock` method to acquire the lock. This call will block the current thread, so it can't do any work until it's our turn to have the lock.  
The call to `lock` would fail if another thread holding the lock panicked.  
After we've acquired the lock, we can treat the return value as mutable reference to the data inside.  
The call to `lock` returns a smart pointer called `MutexGuard`, wrapped in a `LockResult`.  
The `MutexGuard` implements `Deref` to point at our inner data; the smart pointer also has a `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope.  

## Sharing a `Mutex<T>` Between Multiple Threads
`Mutex<T>` cannot be shared in multiple threads in the program because the `Mutex<T>` was moved into the closure.  

## Multiple Ownership with Multiple Threads
Wrap the `Mutex<T>` inside `Rc<T>` to create multiple owners.  
Unfortunately, `Rc<T>` is not safe to share across threads.  

## Atomic Reference Counting with `Arc<T>`
`Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.  
Atomics work like primitive types but are safe to share across threads.  
Thread safety comes with a performance penalty that you only want to pay when you really need to.  

## Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`
`Mutex<T>` provides interior mutability, as the `Cell` family does.  
We use `Mutex<T>` to mutate the contents inside an `Arc<T>`.  
`Mutex<T>` comes with risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.