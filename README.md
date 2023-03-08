# Table of Contents

- [Commands](#commands)
- [Common Concepts](#common-concepts)
    - [Datatypes](#datatypes)
    - [Functions](#functions)
    - [Control Flow](#control-flow)
- [Ownership](#ownership)
- [Structs](#structs)
- [Enums](#enums)
- [Manage Projects](#manage-projects)
    - [Crates](#crates)
    - [Package](#package)
    - [Modules](#modules)
- [Appendix](#appendix)
    - [Operators](#operators)

## Commands

`rustc file_name.rs` => compile the code
`cargo new project_name` => create a project directory
`cargo new --vcs=git` => initialize existing directory
`cargo build` => build cargo project

> This command creates an executable file in `target/debug/project_name` and run this file.

`cargo run` => builds and runs the cargo project
`cargo check` => check if code compiles but doesn't produce executable file
`cargo build --release` => to release the cargo project

> This command creates an executable file in `target/release/project_name` and run this file.

> `Crates.io` is where people in the Rust ecosystem post their open source Rust projects for others to use.

`cargo update` => update crates in the project
`cargo doc --open` => creates a HTML documentation of all crates mentioned in the project

## Common Concepts

### Datatypes

1. Scalar Types
    - Integers
    - Floats
    - Booleans
    - Characters

```markdown
To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`

- Return the `None` value if there is overflow with the `checked_*` methods

- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods

- Saturate at the value’s minimum or maximum values with `saturating_*` methods
```

> char literals uses single quotes, string literals uses double quotes.

2. Compound Types
    - Tuples
    - Arrays

A tuple is a collection of differnt types. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

An array is a collection of same type. Arary have a fixed length. For varied lengths use `Vector`.

### Functions

*snake case* as the conventional style for function and variable names.

**must** declare the type of each parameter.

> **If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.**

### Control Flow

1. Conditional
    - If

> **Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition**

2. Loops
    - loop
    - while
    - for

## Ownership

**Ownership enables Rust to make memory safety guarantees without needing a garbage collector**

### Rules
- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### References

Rust has a feature for using a value without transferring ownership, called *references*.

> References are immutable by default.

> If you have a mutable reference to a value, you can have no other references to that value.

> Same way as above cannot combine mutable and immutable references on same value.

### Slices

Slice is a kind of reference, so it does not have ownership.

> Slices works on both string literals and String

## Structs

There are 3 types:
- Structs
- Tuple Structs
- Unit-Like Structs

## Enums

- Optional Enums
- Control Flow with `if let`

## Manage Projects

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the *module system*, include:

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and **use**: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

### Crates

A crate can come in one of two forms: a binary crate or a library crate. 

**Binary crates** are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs.

**Library crates** don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

### Package

A package is a bundle of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

**Note:**
- A package can contain as many binary crates as you like, but at most only one library crate.
- A package must contain at least one crate, whether that’s a library or binary crate.

### Modules

**Cheat Sheet**

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate) for code to compile.

- **Declaring modules**: In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:

- Inline, within curly brackets that replace the semicolon following `mod garden`
- In the file src/garden.rs
- In the file src/garden/mod.rs

## Appendix

### Operators

<table>
    <thead>
        <tr>
            <th>Operator</th>
            <th>Example</th>
            <th>Explanation</th>
            <th>Overloadable?</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>!</code></td>
            <td><code>ident!(...)</code>, <code>ident!{...}</code>, <code>ident![...]</code></td>
            <td>Macro expansion</td>
            <td></td>
        </tr>
        <tr>
            <td><code>!</code></td>
            <td><code>!expr</code></td>
            <td>Bitwise or logical complement</td>
            <td><code>Not</code></td>
        </tr>
        <tr>
            <td><code>!=</code></td>
            <td><code>expr != expr</code></td>
            <td>Nonequality comparison</td>
            <td><code>PartialEq</code></td>
        </tr>
        <tr>
            <td><code>%</code></td>
            <td><code>expr % expr</code></td>
            <td>Arithmetic remainder</td>
            <td><code>Rem</code></td>
        </tr>
        <tr>
            <td><code>%=</code></td>
            <td><code>var %= expr</code></td>
            <td>Arithmetic remainder and assignment</td>
            <td><code>RemAssign</code></td>
        </tr>
        <tr>
            <td><code>&amp;</code></td>
            <td><code>&amp;expr</code>, <code>&amp;mut expr</code></td>
            <td>Borrow</td>
            <td></td>
        </tr>
        <tr>
            <td><code>&amp;</code></td>
            <td><code>&amp;type</code>, <code>&amp;mut type</code>, <code>&amp;'a type</code>,
                <code>&amp;'a mut type</code></td>
            <td>Borrowed pointer type</td>
            <td></td>
        </tr>
        <tr>
            <td><code>&amp;</code></td>
            <td><code>expr &amp; expr</code></td>
            <td>Bitwise AND</td>
            <td><code>BitAnd</code></td>
        </tr>
        <tr>
            <td><code>&amp;=</code></td>
            <td><code>var &amp;= expr</code></td>
            <td>Bitwise AND and assignment</td>
            <td><code>BitAndAssign</code></td>
        </tr>
        <tr>
            <td><code>&amp;&amp;</code></td>
            <td><code>expr &amp;&amp; expr</code></td>
            <td>Short-circuiting logical AND</td>
            <td></td>
        </tr>
        <tr>
            <td><code>*</code></td>
            <td><code>expr * expr</code></td>
            <td>Arithmetic multiplication</td>
            <td><code>Mul</code></td>
        </tr>
        <tr>
            <td><code>*=</code></td>
            <td><code>var *= expr</code></td>
            <td>Arithmetic multiplication and assignment</td>
            <td><code>MulAssign</code></td>
        </tr>
        <tr>
            <td><code>*</code></td>
            <td><code>*expr</code></td>
            <td>Dereference</td>
            <td><code>Deref</code></td>
        </tr>
        <tr>
            <td><code>*</code></td>
            <td><code>*const type</code>, <code>*mut type</code></td>
            <td>Raw pointer</td>
            <td></td>
        </tr>
        <tr>
            <td><code>+</code></td>
            <td><code>trait + trait</code>, <code>'a + trait</code></td>
            <td>Compound type constraint</td>
            <td></td>
        </tr>
        <tr>
            <td><code>+</code></td>
            <td><code>expr + expr</code></td>
            <td>Arithmetic addition</td>
            <td><code>Add</code></td>
        </tr>
        <tr>
            <td><code>+=</code></td>
            <td><code>var += expr</code></td>
            <td>Arithmetic addition and assignment</td>
            <td><code>AddAssign</code></td>
        </tr>
        <tr>
            <td><code>,</code></td>
            <td><code>expr, expr</code></td>
            <td>Argument and element separator</td>
            <td></td>
        </tr>
        <tr>
            <td><code>-</code></td>
            <td><code>- expr</code></td>
            <td>Arithmetic negation</td>
            <td><code>Neg</code></td>
        </tr>
        <tr>
            <td><code>-</code></td>
            <td><code>expr - expr</code></td>
            <td>Arithmetic subtraction</td>
            <td><code>Sub</code></td>
        </tr>
        <tr>
            <td><code>-=</code></td>
            <td><code>var -= expr</code></td>
            <td>Arithmetic subtraction and assignment</td>
            <td><code>SubAssign</code></td>
        </tr>
        <tr>
            <td><code>-&gt;</code></td>
            <td><code>fn(...) -&gt; type</code>, <code>|...| -&gt; type</code></td>
            <td>Function and closure return type</td>
            <td></td>
        </tr>
        <tr>
            <td><code>.</code></td>
            <td><code>expr.ident</code></td>
            <td>Member access</td>
            <td></td>
        </tr>
        <tr>
            <td><code>..</code></td>
            <td><code>..</code>, <code>expr..</code>, <code>..expr</code>, <code>expr..expr</code></td>
            <td>Right-exclusive range literal</td>
            <td><code>PartialOrd</code></td>
        </tr>
        <tr>
            <td><code>..=</code></td>
            <td><code>..=expr</code>, <code>expr..=expr</code></td>
            <td>Right-inclusive range literal</td>
            <td><code>PartialOrd</code></td>
        </tr>
        <tr>
            <td><code>..</code></td>
            <td><code>..expr</code></td>
            <td>Struct literal update syntax</td>
            <td></td>
        </tr>
        <tr>
            <td><code>..</code></td>
            <td><code>variant(x, ..)</code>, <code>struct_type { x, .. }</code></td>
            <td>“And the rest” pattern binding</td>
            <td></td>
        </tr>
        <tr>
            <td><code>...</code></td>
            <td><code>expr...expr</code></td>
            <td>(Deprecated, use <code>..=</code> instead) In a pattern: inclusive range pattern</td>
            <td></td>
        </tr>
        <tr>
            <td><code>/</code></td>
            <td><code>expr / expr</code></td>
            <td>Arithmetic division</td>
            <td><code>Div</code></td>
        </tr>
        <tr>
            <td><code>/=</code></td>
            <td><code>var /= expr</code></td>
            <td>Arithmetic division and assignment</td>
            <td><code>DivAssign</code></td>
        </tr>
        <tr>
            <td><code>:</code></td>
            <td><code>pat: type</code>, <code>ident: type</code></td>
            <td>Constraints</td>
            <td></td>
        </tr>
        <tr>
            <td><code>:</code></td>
            <td><code>ident: expr</code></td>
            <td>Struct field initializer</td>
            <td></td>
        </tr>
        <tr>
            <td><code>:</code></td>
            <td><code>'a: loop {...}</code></td>
            <td>Loop label</td>
            <td></td>
        </tr>
        <tr>
            <td><code>;</code></td>
            <td><code>expr;</code></td>
            <td>Statement and item terminator</td>
            <td></td>
        </tr>
        <tr>
            <td><code>;</code></td>
            <td><code>[...; len]</code></td>
            <td>Part of fixed-size array syntax</td>
            <td></td>
        </tr>
        <tr>
            <td><code>&lt;&lt;</code></td>
            <td><code>expr &lt;&lt; expr</code></td>
            <td>Left-shift</td>
            <td><code>Shl</code></td>
        </tr>
        <tr>
            <td><code>&lt;&lt;=</code></td>
            <td><code>var &lt;&lt;= expr</code></td>
            <td>Left-shift and assignment</td>
            <td><code>ShlAssign</code></td>
        </tr>
        <tr>
            <td><code>&lt;</code></td>
            <td><code>expr &lt; expr</code></td>
            <td>Less than comparison</td>
            <td><code>PartialOrd</code></td>
        </tr>
        <tr>
            <td><code>&lt;=</code></td>
            <td><code>expr &lt;= expr</code></td>
            <td>Less than or equal to comparison</td>
            <td><code>PartialOrd</code></td>
        </tr>
        <tr>
            <td><code>=</code></td>
            <td><code>var = expr</code>, <code>ident = type</code></td>
            <td>Assignment/equivalence</td>
            <td></td>
        </tr>
        <tr>
            <td><code>==</code></td>
            <td><code>expr == expr</code></td>
            <td>Equality comparison</td>
            <td><code>PartialEq</code></td>
        </tr>
        <tr>
            <td><code>=&gt;</code></td>
            <td><code>pat =&gt; expr</code></td>
            <td>Part of match arm syntax</td>
            <td></td>
        </tr>
        <tr>
            <td><code>&gt;</code></td>
            <td><code>expr &gt; expr</code></td>
            <td>Greater than comparison</td>
            <td><code>PartialOrd</code></td>
        </tr>
        <tr>
            <td><code>&gt;=</code></td>
            <td><code>expr &gt;= expr</code></td>
            <td>Greater than or equal to comparison</td>
            <td><code>PartialOrd</code></td>
        </tr>
        <tr>
            <td><code>&gt;&gt;</code></td>
            <td><code>expr &gt;&gt; expr</code></td>
            <td>Right-shift</td>
            <td><code>Shr</code></td>
        </tr>
        <tr>
            <td><code>&gt;&gt;=</code></td>
            <td><code>var &gt;&gt;= expr</code></td>
            <td>Right-shift and assignment</td>
            <td><code>ShrAssign</code></td>
        </tr>
        <tr>
            <td><code>@</code></td>
            <td><code>ident @ pat</code></td>
            <td>Pattern binding</td>
            <td></td>
        </tr>
        <tr>
            <td><code>^</code></td>
            <td><code>expr ^ expr</code></td>
            <td>Bitwise exclusive OR</td>
            <td><code>BitXor</code></td>
        </tr>
        <tr>
            <td><code>^=</code></td>
            <td><code>var ^= expr</code></td>
            <td>Bitwise exclusive OR and assignment</td>
            <td><code>BitXorAssign</code></td>
        </tr>
        <tr>
            <td><code>|</code></td>
            <td><code>pat | pat</code></td>
            <td>Pattern alternatives</td>
            <td></td>
        </tr>
        <tr>
            <td><code>|</code></td>
            <td><code>expr | expr</code></td>
            <td>Bitwise OR</td>
            <td><code>BitOr</code></td>
        </tr>
        <tr>
            <td><code>|=</code></td>
            <td><code>var |= expr</code></td>
            <td>Bitwise OR and assignment</td>
            <td><code>BitOrAssign</code></td>
        </tr>
        <tr>
            <td><code>||</code></td>
            <td><code>expr || expr</code></td>
            <td>Short-circuiting logical OR</td>
            <td></td>
        </tr>
        <tr>
            <td><code>?</code></td>
            <td><code>expr?</code></td>
            <td>Error propagation</td>
            <td></td>
        </tr>
    </tbody>
</table>