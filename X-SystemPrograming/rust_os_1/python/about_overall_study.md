🔥 GOOD. Now we’re talking interview strategy.

First — important clarification.

---

# 🧠 Why Your Earlier “Program 1” Felt Different

You built a **supervisor** that:

* Restarts child
* Handles signals
* Escalates
* Prevents crash loops

But the actual question says:

> Program 1: forever
> Starts program as background process
> All processes run forever until manually killed
> Should handle multiple instances

That is NOT a supervisor.

That is more like:

```
nohup + &
```

It does NOT say:

* Restart on crash
* Escalation
* Signal forwarding
* Monitor behavior

You over-engineered it 😄
(Which is good for learning — but different from the question.)

---

# 🎯 Real Intent of Program 1 (Interview Mindset)

They are likely testing:

* Can you fork?
* Can you detach?
* Can you run background processes?
* Can you handle multiple instances?
* Can you avoid zombies?
* Can you structure PID tracking?

Not full systemd.

---

# 🧠 Now Let’s Zoom Out

You said:

> I want to be prepared for 10 similar kinds of programs.

Excellent mindset.

Let’s map the entire “process-control interview universe.”

These questions typically test:

---

# 🔥 Category 1 — Process Creation & Backgrounding

Basic → Mid

1. Write a program like `nohup`
2. Run command in background
3. Detach from terminal
4. Ignore SIGHUP
5. Redirect stdout/stderr to file
6. Print child PID
7. Run multiple instances safely

Concepts:

* fork()
* setsid()
* process groups
* SIGHUP
* controlling terminal

---

# 🔥 Category 2 — Process Monitoring

Like your Program 2.

1. Monitor process by name
2. Restart on crash
3. Keep count of restarts
4. Backoff strategy
5. Detect flapping
6. Heartbeat monitor
7. Timeout if stuck

Concepts:

* /proc scanning
* poll() vs wait()
* non-blocking logic
* PID tracking
* race conditions

---

# 🔥 Category 3 — Zombie & Reaping Problems

Very common.

1. Write program that reaps zombies
2. Explain why zombie happens
3. Handle SIGCHLD
4. Implement subreaper
5. Avoid double wait
6. Avoid orphan leaks

Concepts:

* waitpid(-1)
* WNOHANG
* SIGCHLD
* subreaper
* PID 1 behavior

---

# 🔥 Category 4 — Signal Handling

They love this.

1. Graceful shutdown
2. Forward signals to children
3. Escalation (TERM → KILL)
4. Ignore specific signals
5. Custom reload with SIGHUP
6. Timeout handling
7. Signal race conditions

Concepts:

* async safety
* main-thread signals
* signal delivery
* interrupted syscalls

---

# 🔥 Category 5 — Process Trees & Containment

More advanced.

1. Kill entire process tree
2. Prevent daemon escape
3. Use process groups
4. Explain double fork
5. Use prctl subreaper
6. Compare with cgroups
7. PID namespace behavior

Concepts:

* PGID vs SID
* setsid()
* orphan adoption
* cgroups
* containers

---

# 🔥 Category 6 — Resource Control

Mid-level Linux engineer questions.

1. Limit CPU usage
2. Limit memory usage
3. Detect OOM
4. Kill heavy process
5. Monitor file descriptor usage
6. Limit open files
7. Timeout runaway process

Concepts:

* ulimit
* resource module
* rlimit
* /proc stats

---

# 🔥 Category 7 — IPC (Inter-Process Communication)

Common in system roles.

1. Parent-child pipe communication
2. Send signal on specific message
3. Shared memory
4. UNIX domain socket
5. Simple job queue
6. Worker pool
7. Task dispatcher

Concepts:

* pipes
* select
* epoll
* socketpair
* multiprocessing

---

# 🔥 Category 8 — Mini Init System

Advanced but possible.

1. PID 1 inside container
2. Reap all zombies
3. Forward signals properly
4. Shutdown sequence ordering
5. Restart policy
6. Dependency ordering
7. Health checks

You already touched this territory.

---

# 🎯 Now Specifically For THIS Assignment

The two programs together test:

## Program 1 (forever)

* Background execution
* Process independence
* Multiple instances
* Possibly PID tracking

## Program 2 (monitor)

* Process detection
* Monitoring
* Restart logic
* Matching processes started by forever
* Avoid duplicate restarts
* Keep running continuously

The tricky part is:

👉 How does monitor know which processes were started by forever?

That’s the real interview trap.

---

# 🚨 The Hidden Hard Part

They didn’t say:

* Use PID file
* Use shared memory
* Use database
* Use lock file
* Use naming convention

So you must design:

How will forever mark the processes it starts?

Options:

1. Write PID file
2. Add custom environment variable
3. Wrap command with unique identifier
4. Store metadata file
5. Use process title
6. Use a registry file

This is the real design question.

---

# 🧠 Before We Write Any Code

Let’s clarify architecture.

We must decide:

1. How forever records processes
2. How monitor identifies them
3. How monitor restarts them safely
4. How to prevent duplicate restarts
5. How to avoid race conditions

This is where interviews separate coders from engineers.

---

# 🔥 So Before Moving Forward

Answer me this:

If you were designing forever + monitor from scratch…

How would you let monitor identify ONLY the processes started by forever?

Without relying on fragile name matching like:

```
ps | grep sleep
```

Think like an engineer.

Once you answer that,
we design both programs properly — at same depth as before.


## 🔹 1️⃣ Basic daemon wrapper

Run program in background and detach properly.

---

## 🔹 2️⃣ Auto restart on crash

Like `monitor` but inside same program.

---

## 🔹 3️⃣ Restart with max retry count

Restart only 3 times then stop.

---

## 🔹 4️⃣ Restart with exponential backoff

1s → 2s → 4s → 8s

Very common SRE question.

---

## 🔹 5️⃣ PID file implementation

Store child PID in file.
Prevent duplicate runs.

---

## 🔹 6️⃣ Only one instance allowed

If already running → exit.

---

## 🔹 7️⃣ Graceful shutdown handling

Forward SIGTERM to child.

---

## 🔹 8️⃣ Timeout execution

Kill child if running longer than X seconds.

---

## 🔹 9️⃣ Logging wrapper

Redirect output to log file.

---

## 🔹 🔟 Process tree cleanup

Kill entire process group.

---

## 🔹 1️⃣1️⃣ Monitor multiple commands

Run 3 commands and restart individually.

---

## 🔹 1️⃣2️⃣ Simple mini-systemd

Config file with services and restart policies.

---
