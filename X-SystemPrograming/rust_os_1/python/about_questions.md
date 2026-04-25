
>    Exact 2 Coding Questions (verbatim from real InfStones Application Engineer round)
>    Program 1: forever
>
>    Takes one argument (the name of a program to run, e.g. ./forever sleep 10).
>    Every time you run it, it starts that program as a background process with a unique PID.
>    All processes run forever until manually killed.
>    Should handle multiple instances.
>
>    Program 2: monitor
>
>    Takes one argument — the name of the program it should watch (the same one forever starts).
>    Continuously checks if any process started by forever dies.
>    If any dies, automatically restarts it with the same command.
>    Should keep running and monitoring.
>
>    (You upload both files or paste both in the platform.)
>
>    one more thign before moving forward
>    we started with program  1 and buiold someting but that is different form the quesiton 1 i dont know why i felt like that? 
>    also we have to go through the similar kind of sinereos, even if the question are conform i want to be prepared for atleast more 10 thiks kind of programs, 
>    what are the other things they can give in this program lets discuss that first then we will move with others. what are the thigns we can do here? all basic to mid basic level questions.

==========================================================

Now it makes sense why Program 1 was called `forever`.
This is not just process spawning — this is about:

* Process discovery
* Process tracking
* Inter-process coordination
* External monitoring
* Restarting detached processes
* Multi-instance handling

This is much more interesting than Program 1 alone.

Let’s break it down properly.

---

# 🔥 What They Are Actually Testing

They are testing whether you understand:

1. Process lifecycle
2. Background processes
3. How to find processes by command
4. How to distinguish *your* processes vs random ones
5. How to monitor external processes
6. How to restart them safely
7. How to handle multiple instances

This is a simplified version of:

* systemd
* supervisord
* pm2
* Kubernetes controller loop

---

# 🧠 Program 1 — `forever`

### Requirements:

* Takes one argument → program to run
* Starts it in background
* Multiple instances allowed
* Processes live until manually killed
* Unique PID each time (naturally true)

### Important subtlety:

It must:

* Not block
* Not monitor
* Just spawn and exit
* Work even if monitor isn’t running

This means:

`forever` is only a launcher.

---

# 🧠 Program 2 — `monitor`

### Requirements:

* Takes program name
* Watches all processes started by `forever`
* If any dies → restart it
* Keep running forever

This is the tricky one.

Because now:

You need to detect:

> “Which processes were started by forever?”

Without that, you can’t distinguish:

```bash
sleep 10
```

started manually vs started by forever.

This is the key design problem.

---

# 🚨 The Real Challenge

How does `monitor` know:

* Which PIDs belong to forever?
* With what arguments they were started?
* How many instances exist?
* Which ones died?

You cannot rely on process name alone.
Because someone could run the same command manually.

---

# 🧠 So We Need A Tracking Strategy

There are 3 realistic approaches:

---

# ✅ Option 1 — PID File Registry (Clean Design)

When `forever` launches a process:

* It writes:

  * PID
  * Command
  * Arguments
* Into a file

Example:

```
/tmp/forever_sleep.registry
```

Monitor:

* Reads registry
* For each PID:

  * Check if alive
  * If dead → restart
  * Update registry

This is the cleanest solution.

---

# ✅ Option 2 — Use Process Groups

When forever launches:

* Create a unique process group
* Tag it somehow

But you still need metadata storage.

---

# ❌ Option 3 — Scan Entire System With `ps`

Bad design.

You could do:

```bash
ps aux | grep sleep
```

But that’s unreliable and sloppy.

In an interview, that’s weak.

---

# 🎯 Correct Architectural Approach

Use a **registry file**.

Each `forever` run appends an entry:

Example format:

```
PID: 12345
CMD: sleep 10
```

Or JSON:

```json
{
  "pid": 12345,
  "command": ["sleep", "10"]
}
```

Monitor reads and maintains this.

---

# 🔥 Important Question

What happens if:

1. monitor is not running
2. forever starts processes
3. some die
4. monitor starts later

Should it restart them?

Yes.

So monitor must:

* Compare registry vs actual running PIDs
* Restart missing ones

---

# 🧠 Deep Design Questions

### What if PID gets reused?

Linux reuses PIDs.

If process dies and PID reused,
monitor might think old process still alive.

Solution:

Store:

* PID
* Start time (from /proc/PID/stat)
* Or process creation time

Then verify both.

This is advanced.

---

# 🧠 How Monitor Checks If Process Is Alive

Simplest method:

```python
os.kill(pid, 0)
```

If no exception → alive
If OSError → dead

This is standard technique.

---

# 🔥 Now Let’s Design Both Programs Properly

---

# 🟢 Program 1: forever

Responsibilities:

* Launch command in background
* Write registry entry
* Exit immediately

Basic design:

```python
child = subprocess.Popen(command)
append_to_registry(child.pid, command)
```

Do NOT wait.

---

# 🔵 Program 2: monitor

Responsibilities:

* Infinite loop
* Read registry
* For each entry:

  * Check if alive
  * If dead → restart
  * Update PID
* Sleep 1–2 seconds
* Repeat

---

# ⚠️ Edge Cases

1. Registry corruption
2. Simultaneous writes
3. Process started outside forever
4. Duplicate entries
5. Registry file deletion
6. Race conditions

For interview, simple but correct solution is fine.

---

# 🚀 Now Let Me Ask You Something Important

Suppose someone runs:

```bash
./forever sleep 10
./forever sleep 10
./forever sleep 10
```

Now 3 instances exist.

If one dies:

Monitor must restart ONLY that one.

So registry must contain separate entries.

Agree?

---

# 🧠 One More Subtle Thing

What if someone manually kills one process?

Monitor should restart it.

What if someone manually kills monitor?

Processes continue running.

That’s correct design.

---

# 🎯 Now Here’s What I Want From You

Before I write implementation:

Answer this:

Where should registry file live?

* Current directory?
* /tmp?
* /var/run?
* Hidden file?
* Per-program registry?

Your design decision matters.

Think like production engineer.

