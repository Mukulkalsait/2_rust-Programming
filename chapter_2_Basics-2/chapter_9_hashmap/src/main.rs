use chapter_9_hashmap::map_functions::create_hash_map::create_hash_map;
use chapter_9_hashmap::map_functions::operation_in_hash_map::remove_item_from_hash_map;
use chapter_9_hashmap::map_functions::printing_hash_map::print_hash_methods;

fn main() {
    // G: Tupil HashMap return to mutable.
    let mut maps = create_hash_map();

    // G: passing as mutable reference.
    remove_item_from_hash_map(&mut maps[1]);

    // G: passing as immutable reverence.
    print_hash_methods(&maps[1]);

    /* |------------------------------------------------------------------------------------------------------------
     * IMP : The Mutable reference + immutable reference conclusion.                                                |
     * |------------------------------------------------------------------------------------------------------------
     *
     * G: Tupil HashMap return to mutable.
     *   let mut maps = create_hash_map();
     *
     * G: passing as mutable reference.
     *   remove_item_from_hash_map(&mut maps[1]);
     *
     * G: passing as immutable reverence.
     *   print_hash_methods(&maps[1]);
     */
}

/*
 * |------------------------------------------------------------------------------------------------------------
 * Y: Harshible and harsh types.                                                                               |
 * |------------------------------------------------------------------------------------------------------------
 *  “Hashable” — What Does It Really Mean?
 *  When I say a key type must be hashable, it means:
 *  The type must be able to produce a hash value (an integer) that represents its contents.
 *  That hash value is used internally by the HashMap to decide where to store the key-value pair in memory so it can retrieve it quickly.
 *  In Rust, this means the type must implement the Hash trait.
 *  Hash in data structures
 *  This is not the same thing as password hashing (like bcrypt or Argon2), which is designed to be slow and irreversible for security.
 *  Hashing for a HashMap is fast and deterministic — same input → same hash → quick lookup.
 *  Example:
 *  "alice"  → hash function → 932847232
 *  "bob"    → hash function → 120398473
 *
 * |------------------------------------------------------------------------------------------------------------
 * DX: Common Confussion in HASHMAP.                                                                            |
 * |------------------------------------------------------------------------------------------------------------
 *     1. Ownership & Borrowing: Strings & complex keys are moved unless you pass references.
 *     2. Copy vs Clone: Integers copy by default, Strings must be cloned if you want to reuse after inserting.
 *     3. Order is not guaranteed — HashMap is unordered.
 *     4. .get() returns a reference → sometimes you must dereference it with *.
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 */
