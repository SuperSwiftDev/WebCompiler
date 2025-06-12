## 🔧 Overview

This document outlines the core control attributes, compile-time semantics, and contextual evaluation model for a generalized HTML-oriented DSL designed for static site generation. Inspired by macro systems in modern frontend frameworks (Svelte, Astro, Vue, JSX), this DSL embraces declarative data binding, control flow, and compile-time evaluation without runtime JavaScript dependencies.

The system is format-agnostic (e.g., works with XML or extended HTML) and is suitable for static site compilers written in languages like Rust or Go.

---

## 🔀 Control Attributes

| Attribute      | Type    | Purpose                                                      |
| -------------- | ------- | ------------------------------------------------------------ |
| `from`         | string  | Substitutes content/value from scoped data path              |
| `iterate`      | string  | Repeats node over each item in the given collection          |
| `if`           | boolean | Conditionally includes node when expression is truthy        |
| `unless`       | boolean | Conditionally excludes node when expression is truthy        |
| `bind:x="..."` | binding | Introduces a scoped variable                                 |
| `fallback`     | string  | Supplies default value if `from` path is missing or null     |
| `key`          | string  | Stable ID for repeated nodes (for diffing or identification) |

---

## 🔎 Attribute Semantics

### `from="path.to.value"`

#### Format

* **Formal grammar:**

  ```ebnf
  path        ::= identifier ("." identifier)*
  identifier  ::= [a-zA-Z_][a-zA-Z0-9_]*
  ```
* Leading dot (`.`) may refer to the current context when used alone.

#### Semantics

* Resolves the path relative to the current scope.
* Each `identifier` must resolve to a key in a dictionary-like structure (e.g., JSON object).
* If the path does not resolve or points to null/undefined:

  * If `fallback` is present, it is used.
  * Otherwise, the node is considered incomplete or omitted based on compiler policy.

#### ✅ Examples (Valid)

| Expression           | Meaning                                  |
| -------------------- | ---------------------------------------- |
| `title`              | Fetches `title` from the current context |
| `product.name`       | Fetches nested `name` inside `product`   |
| `.`                  | Refers to the current loop item          |
| `loop.index`         | Refers to loop iteration index           |
| `site.contact.email` | Fetches deeply nested global value       |

#### ❌ Examples (Invalid by Default)

| Expression            | Reason                              |
| --------------------- | ----------------------------------- |
| `items[0]`            | No bracket/index syntax allowed     |
| `user?.name`          | No optional chaining supported      |
| `count - 1`           | No arithmetic expressions in `from` |
| `getUserName()`       | No function invocation permitted    |
| `title ?? "Untitled"` | Null-coalescing logic not supported |

---

## 🧮 Expression Grammar (Generalized)

To support conditional logic, bindings, and future extensibility, the following expression grammar is proposed:

```ebnf
expression    ::= logical_or
logical_or    ::= logical_and ("||" logical_and)*
logical_and   ::= equality ("&&" equality)*
equality      ::= comparison (("==" | "!=") comparison)*
comparison    ::= term (("<" | ">" | "<=" | ">=") term)*
term          ::= factor (("+" | "-") factor)*
factor        ::= primary (("*" | "/") primary)*
primary       ::= path | number | boolean | string | "(" expression ")"
path          ::= identifier ("." identifier)*
identifier    ::= [a-zA-Z_][a-zA-Z0-9_]*
number        ::= [0-9]+ ("." [0-9]+)?
boolean       ::= "true" | "false"
string        ::= '"' .*? '"' | '\'' .*? '\''
```

* **Note**: Only a restricted subset of this grammar may be supported per attribute type.

  * `from` → limited to `path`
  * `bind:x` → allows full `expression`
  * `if` / `unless` → boolean-producing `expression`

This ensures simple use cases remain ergonomic, while still allowing powerful compile-time evaluation.

---

## 🔁 Other Attribute Semantics

### `iterate="items"`

* Expands the node’s children once per item in `items` (array expected).
* Each iteration gets its own scope:

  * `.` → current item
  * `loop.index`, `loop.first`, `loop.last`, `loop.length`

### `if="condition"` and `unless="condition"`

* Accept expressions that resolve to boolean values.
* Expressions are evaluated within the current scope.

### `bind:x="expression"`

* Evaluates a valid expression.
* Binds the result to a local variable `x`.
* Variable is scoped to the current element’s children.

### `fallback="Static content"`

* Injected only when `from` fails or evaluates to null.
* Used for progressive enhancement or placeholder content.

### `key="expression"`

* Used to assign a stable ID to repeated nodes (e.g. for DOM diffing or caching).
* Typically points to a unique field like `id` within each item.

---

## 🧠 Contextual Bindings

Contextual bindings are variables available at compile time for use within control expressions such as `from`, `if`, `bind`, and `key`. These enable expressive and deterministic layout expansion.

| Binding       | Type   | Scope          | Description                       |
| ------------- | ------ | -------------- | --------------------------------- |
| `.`           | any    | `iterate` body | Current loop item                 |
| `loop.index`  | int    | `iterate` body | Zero-based index                  |
| `loop.first`  | bool   | `iterate` body | True on first iteration           |
| `loop.last`   | bool   | `iterate` body | True on last iteration            |
| `loop.length` | int    | `iterate` body | Total number of iterations        |
| `bind:x`      | any    | lexical        | Named variable from parent `bind` |
| Global scope  | object | top-level      | Static or dynamic global data     |

### ✅ Examples

#### 1. Basic iteration using `.` and `loop.index`

```html
<card iterate="services">
  <h2 from=".title"/>
  <p from=".description"/>
  <small>Item #[from="loop.index"]</small>
</card>
```

#### 2. Use `loop.first` to mark the first item

```html
<feature iterate="features">
  <h2 from="name"/>
  <badge if="loop.first">Popular</badge>
</feature>
```

#### 3. Alias loop item with `bind`

```html
<user iterate="users" bind:person=".">
  <p from="person.full_name"/>
</user>
```

#### 4. Custom loop ID using `key`

```html
<testimonial iterate="testimonials" key="id">
  <blockquote from="quote"/>
</testimonial>
```

#### 5. Conditional rendering with `if` on loop metadata

```html
<step iterate="steps">
  <p from="text"/>
  <note if="loop.last">You're done!</note>
</step>
```

These examples illustrate how contextual bindings can simplify repetition, conditionals, and scoping in the DSL while remaining statically evaluable and predictable.

These are available within `iterate` or local scopes:

| Binding       | Type   | Scope          | Description                       |
| ------------- | ------ | -------------- | --------------------------------- |
| `.`           | any    | `iterate` body | Current loop item                 |
| `loop.index`  | int    | `iterate` body | Zero-based index                  |
| `loop.first`  | bool   | `iterate` body | True on first iteration           |
| `loop.last`   | bool   | `iterate` body | True on last iteration            |
| `loop.length` | int    | `iterate` body | Total number of iterations        |
| `bind:x`      | any    | lexical        | Named variable from parent `bind` |
| Global scope  | object | top-level      | Static or dynamic global data     |

---

## ⚙️ Evaluation Strategy

1. Evaluate `if` / `unless` guards.
2. Expand nodes using `iterate`, establishing a scope for each iteration.
3. Within each scope:

   * Apply `bind` assignments
   * Resolve `from` path to produce content or attribute value
   * If missing, try `fallback`
4. Output is compiled to a static, pre-evaluated document tree.

