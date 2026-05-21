Perfect idea.
Fast repetition is EXACTLY how type intuition becomes automatic.

---

# 🔥 Rust Ownership & Option Revision Pack

For EACH test answer quickly:

```text
1. Type transformations
2. Ownership or borrow?
3. What moved?
4. What prints / compiles?
```

---

# TEST 1 — `as_ref()`

```rust id="h4v4o3"
fn main() {
    let x = Some(String::from("hello"));

    let y = x.as_ref();

    println!("{:?}", x);
}
```

Questions:

1. Type of `x`?
2. Type of `y`?
3. Did ownership move?
4. Does println compile?

T1: 
Type of x? Option<String>
Type of y? Optoin<&String>
Did ownership move? no 
Does println compile? yes and it also consumes the x.

```text id="hsmgk1"
println also consumes x
```
❌ No. `println!` usually BORROWS. does NOT consume ownership.

# TEST 2 — `unwrap()`

```rust id="hz6nlz"
fn main() {
    let x = Some(String::from("hello"));

    let y = x.unwrap();

    println!("{}", y);
}
```

Questions:

1. Type of `y`?
2. Who owns String now?
3. Can `x` still be used?

T2:
Type of y? String
Who owns String now? y is owner
Can x still be used? no
---

# TEST 3 — `as_ref().unwrap()`

```rust id="xfqg2n"
fn main() {
    let x = Some(String::from("hello"));

    let y = x.as_ref().unwrap();

    println!("{}", x.unwrap());
}
```

Questions:

1. Type of `y`?
2. Did unwrap move ownership here?
3. Why does final println still work?

T3:
Type of y? &String 
Did unwrap move ownership here? no because its used with as_ref
Why does final println still work? yes. 
---

# TEST 4 — `take()`

```rust id="uwzvsr"
fn main() {
    let mut x = Some(String::from("hello"));

    let y = x.take();

    println!("{:?}", x);
    println!("{:?}", y);
}
```

Questions:

1. Type of `y`?
2. What happened to `x`?
3. Why must `x` be mutable?

T4:
Type of y? String
What happened to x? safely replaced with None value
Why must x be mutable? to use .take function x must be Mutable. as it safely replace the value with None.

❌ Tiny correction.
You said: y = string NO. Remember:
take() returns: Option<String>
NOT `String`. Because it moves entire optional value.|  Some(String) -> moved as Some(String)

---

# TEST 5 — `map()`

```rust id="nsmjlwm"
fn main() {
    let x = Some(5);

    let y = x.map(|n| n * 2);

    println!("{:?}", y);
}
```

Questions:

1. Type of `y`?
2. Did map unwrap Option?
3. What prints?

T5:
Type of y? useze 
Did map unwrap Option? yes
What prints? 10 

❌ Important correction.
Type of y = usize = NO.
`map()` keeps container. = Option<i32>

Did map unwrap option? yes ❌ Not exactly.
It TRANSFORMS inside Option. Container remains.

---

# TEST 6 — `?`

```rust id="72qvwy"
fn get() -> Option<String> {
    let x = Some(String::from("hello"));

    let y = x?;

    Some(y)
}
```

Questions:

1. Type of `y`?
2. Does `?` move or borrow?
3. What happens on `None`?
T6: 
Type of y? String.(if fail early return)
Does ? move or borrow? it moves.
What happens on None? (it does early return, the code did not move aheade and return trigger on same line)
some(y) mean we are returning Optins<String> again, this proves that ? moves and not borrow. 

---

# TEST 7 — `Box + deref`

```rust id="6mjlwm"
fn main() {
    let x = Box::new(String::from("hello"));

    println!("{}", x.len());
}
```

Questions:

1. Why does `.len()` work on Box?
2. Did ownership move?
3. What feature enables this?


T7: 
Why does .len() work on Box? yes.
Did ownership move? yes... i tingk ? did it?
What feature enables this? Box is basicaly a smpart pointer which did this:
save the values in heap while saving the pointer and length in stack, but i dont remember aout the .len() specifically,so we will practise this part more from now on. 

#### `.len()` works because of: Deref coercion NOT because Box stores heap data.
>  What Rust does automatically
```text
Box<String> -> String  // ❌ Did ownership move? NO.
```

> `.len()` signature is:
```rust
fn len(&self)
```

---

# TEST 8 — consuming method

```rust id="gpkb2f"
fn main() {
    let x = String::from("hello");

    let y = x.into_bytes();

    println!("{:?}", y);
}
```

Questions:

1. Does `into_bytes()` consume ownership?
2. Can `x` still be used?
3. Signature style?

   * `self`
   * `&self`
   * `&mut self`

T8: 
Does into_bytes() consume ownership? yes it did, but i just found that out will practice this alsoo more 
Can x still be used? if moved we cannot use x 
Signature style?  if its move then sighature stype of .into_bytes(self) is this.
✅ into_ usually means ownership transfer.

---

# TEST 9 — mutable borrow

```rust id="dfv4a2"
fn main() {
    let mut x = String::from("hello");

    x.push_str(" world");

    println!("{}", x);
}
```

Questions:

1. Did ownership move?
2. Why mutable?
3. Signature style?
T9: 
Did ownership move? no ownership did not moved. 
Why mutable? we are changing x so mutable, 
Signature style? i dont know this may be &mut self 
```rust
push_str(&mut self) //  Exactly.
```

---

# TEST 10 — linked-list traversal

```rust id="qt1e77"
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let head = Some(Box::new(Node {
        value: 1,
        next: None,
    }));

    let current = head.as_ref().unwrap();

    println!("{}", current.value);
}
```

Questions:

1. Type of `current`?
2. Why no ownership move?
3. Can `head` still be used?

T10: ok this code deserves more questions, 
type of head= Options<Box<Node>>
Type of current? Options<&Box<Node>> but with .unwrap it becomes = &Box<Node>
Why no ownership move? as_ref() used 
Can head still be used? yes 100% it can be used. 

---

# TEST 11 — mutable traversal

```rust id="wsw88l"
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut head = Some(Box::new(Node {
        value: 1,
        next: None,
    }));

    let current = head.as_mut().unwrap();

    current.value = 99;
}
```

Questions:

1. Type of `current`?
2. Why `.as_mut()`?
3. Why modification works?

T11: 
Type of current? &mut Box<Node>
Why .as_mut()? bcause we care chaing value , updating node so as_mut() needed. 
Why modification works? we are tkaing mutable reference so modification works as well as the head is mutable. 



---

# TEST 12 — borrow vs move

```rust id="1njlwm"
fn main() {
    let x = String::from("hello");

    let y = &x;

    println!("{}", x);
}
```

Questions:

1. Did ownership move?
2. Who owns String?
3. Why compile succeeds?


T12: 
Did ownership move? no not moved 
Who owns String? x is owner
Why compile succeeds? because x is not moved. 

---

# TEST 13 — move trap

```rust id="8cy4p9"
fn main() {
    let x = String::from("hello");

    let y = x;

    println!("{}", x);
}
```

Questions:

1. What moved?
2. Why final println fails?
3. Who owns String now?
T13:
Questions:

What moved? x moved to y
Why final println fails? x moved and we are trying to get x value
Who owns String now? y is owner



---

# TEST 14 — nested transformation

```rust id="mjlwm2"
fn main() {
    let x = Some(Box::new(String::from("hello")));

    let y = x.as_ref().unwrap();

    println!("{}", y.len());
}
```

Questions:

1. Type of `y`?
2. Why `.len()` works?
3. Who owns String?
T14:
type of x = Option<Box<String>>
Type of y? = &Box<Strging>
Why .len() works? yes, its box method, 
Who owns String? x owns string


```text
len is box method // ❌ NO.
```
`Box` has no `.len()`. Rust deref-coerces: &Box<String> -> &String

Then calls String’s `.len()`.

---

# TEST 15 — most important mental model

Fill transformation chain:

```text id="tljlwm"
Option<String>
-> as_ref()
-> ?
-> ?
```

Goal:

```text id="39jlwm"
&String
```

And:

```text id="krjlwm"
Option<Box<Node>>
-> as_mut()
-> ?
-> ?
```

Goal:

```text id="h2jlwm"
&mut Node
```


T15: a
goal:  & string 
Option<String>
-> as_ref().unwrap()

b: goal : &mut Node

Options<Box<Node>>
as_mut().unwrap()
wait how do we take this tigns out of box? i forgot...

ANSWER: how do we take things out of Box? Usually: # YOU DON'T.

You usually: * borrow through Box deref through Box
---

####### Example

```rust id="5jlwmn"
&Box<Node>
// can automatically behave like: through deref coercion.
&Node
```
# So:

```rust id="qjlwme"
Option<Box<Node>>
-> as_mut()
-> Option<&mut Box<Node>>
-> unwrap()
-> &mut Box<Node>
```

AND THEN:

```text id="tjlwmf"
&mut Box<Node>
```

automatically behaves like:

```text id="jjlwmr"
&mut Node
```

# THIS is why linked-list traversal works so smoothly in Rust.

Without deref coercion,
linked lists would be horrible.

