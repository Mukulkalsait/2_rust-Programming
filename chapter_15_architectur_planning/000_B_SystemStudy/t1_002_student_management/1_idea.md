# 🎮 Rust Backend Architecture Game

## Stage 1 — System Skeleton

You are designing the backend for a **Student Management System**.

Imagine this is a real backend that a school/college could actually use. Don't worry about HTTP, databases, frameworks, or deployment yet. We're starting at the **domain level**.

### Essential requirements

The system should eventually be able to handle:

* Students
* Teachers

* Basic information
* attendance
* Classes or enrollments
* Exams and marks

* Courses/subjects

* Academic relationships between students, teachers, and subjects

Some basic rules:

* A student can study multiple subjects.
* A subject can have multiple students.
* A teacher can teach multiple subjects.
* Students receive marks for exams/assessments.
* Attendance is recorded for students.
* Some information should be validated rather than accepting arbitrary values.
* The system will eventually have multiple users/requests interacting with it concurrently.

### Important constraints

For now:

* **Rust backend**
* Don't think about REST endpoints yet.
* Don't introduce traits just because Rust has traits.
* Don't introduce `Arc`, `Mutex`, etc. unless your design gives us a reason.
* We will decide those later.

Think in terms of:

> **What things exist in this system, and what responsibilities do they have?**

---

## Your turn — Stage 1

Identify the **major components/entities** you think this system needs.

For each one, give me roughly:

```text
Component:
Why it exists:
What it is responsible for:
```

Don't worry about fields, Rust types, ownership, traits, or methods yet.

**You design the skeleton first.** I'll review it and challenge your decisions before we move to Stage 2.

