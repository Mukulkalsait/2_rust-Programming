
## Schema Evolution

Rust structs and database schemas are separate. Changing a Rust struct, such as renaming `Student` or changing `student_id` to `student`, does **not automatically change the SQLite database**. Database changes are handled explicitly through **migrations**, which evolve the schema while preserving existing data whenever possible.

## Migrations

Each database change gets a migration, such as adding, removing, or renaming columns. Migrations are applied sequentially (`v1 → v2 → v3...`), allowing existing databases to safely reach the latest schema. Some changes can require data transformation or special handling, but simply renaming a Rust struct generally has no database impact.

## Rust Model vs Database

A Rust struct represents how our application wants to work with data. A database represents how that data is stored and related. They can look similar, but **they don't have to match exactly**.

## Relationship Table

When two entities have a relationship such as `Class → Students`, we normally don't store `Vec<StudentId>` in one database column. Instead, we create a separate table such as `class_students` containing `class_id` and `student_id`.

## Adding & Removing Relationships

To add a student to a class, we insert a row into `class_students`. To remove them, we delete that specific row using `class_id` + `student_id`. We never need to modify a giant array.

## Foreign Keys

The database knows what belongs to what through **foreign keys**. `class_students.class_id` references `classes.id`, while `class_students.student_id` references `students.id`. This lets SQLite enforce valid relationships.

## Migrations

We create relationship tables through **database migrations**. When the application is installed or updated, migrations create or modify the required tables automatically.

## Repository / Database Layer

Our application won't directly manipulate relationship tables everywhere. A database/repository layer will handle operations such as `add_student`, `remove_student`, and `get_students`. The domain can continue working with something convenient like `Vec<StudentId>`.
