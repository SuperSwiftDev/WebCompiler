# HTML-Oriented DSL: Control API Specification

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
* The current scope is always a dictionary mapping, so every key must be referenced explicitly. There is no special leading dot (`.`) context.

#### Semantics

* Resolves the path relative to the current lexical environment.
* All references must be made through declared keys; no implicit reference to `.` is allowed.
* This simplifies resolution and avoids ambiguity in nested scopes.

#### ✅ Examples (Valid)

| Expression           | Meaning                                  |
| -------------------- | ---------------------------------------- |
| `title`              | Fetches `title` from the current context |
| `product.name`       | Fetches nested `name` inside `product`   |
| `item`               | Full item passed via `bind:item="."`     |
| `loop.index`         | Refers to loop iteration index           |
| `site.contact.email` | Fetches deeply nested global value       |

#### ❌ Examples (Invalid by Design)

| Expression      | Reason                              |
| --------------- | ----------------------------------- |
| `.`             | No implicit scope value allowed     |
| `items[0]`      | No bracket/index syntax allowed     |
| `user?.name`    | No optional chaining supported      |
| `count - 1`     | No arithmetic expressions in `from` |
| `getUserName()` | No function invocation permitted    |

---

## 🧠 Contextual Bindings

Contextual bindings are variables available at compile time for use within control expressions such as `from`, `if`, `bind`, and `key`. These enable expressive and deterministic layout expansion.

| Binding       | Type   | Scope          | Description                       |
| ------------- | ------ | -------------- | --------------------------------- |
| `loop.index`  | int    | `iterate` body | Zero-based index                  |
| `loop.first`  | bool   | `iterate` body | True on first iteration           |
| `loop.last`   | bool   | `iterate` body | True on last iteration            |
| `loop.length` | int    | `iterate` body | Total number of iterations        |
| `bind:x`      | any    | lexical        | Named variable from parent `bind` |
| Global scope  | object | top-level      | Static or dynamic global data     |

### ✅ Examples

#### 1. Basic iteration with explicit binding

```html
<card iterate="services" bind:item="item">
  <h2 from="item.title"/>
  <p from="item.description"/>
  <small>Item #[from="loop.index"]</small>
</card>
```

#### 2. Marking the first item using `loop.first`

```html
<feature iterate="features" bind:feature="feature">
  <h2 from="feature.name"/>
  <badge if="loop.first">Popular</badge>
</feature>
```

#### 3. Nested binding

```html
<entry iterate="posts" bind:post="post">
  <section iterate="post.comments" bind:comment="comment">
    <p from="comment.text"/>
  </section>
</entry>
```

#### 4. Unique key based on scoped value

```html
<testimonial iterate="testimonials" bind:t="t" key="t.id">
  <blockquote from="t.quote"/>
</testimonial>
```

These examples reflect the assumption that all scoped data must be explicitly named and that the environment is always a key-value mapping, ensuring clarity and simplicity in both mental model and implementation.

---

## 🧩 Include Semantics and Hygiene

The `<include src="component.html" bind:... />` mechanism allows a static fragment to be included and optionally parameterized. Its semantics resemble a hygienic macro system: values passed in must resolve **within the including (host) context**, not the included file's scope.

### 📌 Hygiene Principles

* **Includes are lexically resolved at the call site**.
* **Bindings must be explicit** (e.g., `bind:title="title"`).
* The included file may reference these passed bindings as local identifiers.
* **No access to the including file's scope is permitted unless passed explicitly**.

### ✅ Example: Parameterized Include

```html
<include src="component.html" bind:title="pageTitle" bind:show="loop.first" />
```

If `component.html` contains:

```html
<header>
  <h1 from="title"/>
  <badge if="show">NEW</badge>
</header>
```

…it will render with `title` and `show` populated from the caller’s scope. This ensures **predictable, hygienic evaluation**, where components cannot implicitly reach into enclosing variables.

### 🚫 Anti-Example (Disallowed Implicit Leakage)

```html
<!-- BAD: this would not work -->
<include src="component.html"/>
<!-- component.html internally tries to access `title` from caller -->
```

Instead, make the bindings explicit.

### 🔍 Static Include Validation

To support tooling, linting, and correctness checking, the DSL encourages or requires component files to declare their parameter expectations.

#### Option 1: Inline Declaration

```html
<!-- @params title: string, show: boolean -->
<header>
  <h1 from="title"/>
  <badge if="show">NEW</badge>
</header>
```

#### Option 2: External Metadata File (e.g., `component.html.meta`)

```json
{
  "params": {
    "title": "string",
    "show": "boolean"
  }
}
```

#### Option 3: Inferred Contract (fallback)

* Compiler scans the component for top-level `from`/`if` references.
* All undeclared identifiers treated as expected bindings.
* Less reliable; may produce false positives if fallback or optional fields are present.

### 🛠 Compiler Behavior

* At include time:

  * Load or infer expected parameters
  * Check that all required bindings are passed
  * Optionally warn on extra bindings
* Enables: IDE autocomplete, validation, and stable refactoring.

### 🔒 Summary

* Like Rust’s hygienic macros, `include` **does not inherit host scope implicitly**.
* Values must be **declared via `bind:x="expression"`** at the call site.
* Components should either declare parameters explicitly or be inferred by the compiler.
* This approach supports tooling, static validation, and modular reuse.

The `<include src="component.html" bind:... />` mechanism allows a static fragment to be included and optionally parameterized. Its semantics resemble a hygienic macro system: values passed in must resolve **within the including (host) context**, not the included file's scope.

### 📌 Hygiene Principles

* **Includes are lexically resolved at the call site**.
* **Bindings must be explicit** (e.g., `bind:title="title"`).
* The included file may reference these passed bindings as local identifiers.
* **No access to the including file's scope is permitted unless passed explicitly**.

### ✅ Example: Parameterized Include

```html
<include src="component.html" bind:title="pageTitle" bind:show="loop.first" />
```

If `component.html` contains:

```html
<header>
  <h1 from="title"/>
  <badge if="show">NEW</badge>
</header>
```

…it will render with `title` and `show` populated from the caller’s scope. This ensures **predictable, hygienic evaluation**, where components cannot implicitly reach into enclosing variables.

### 🚫 Anti-Example (Disallowed Implicit Leakage)

```html
<!-- BAD: this would not work -->
<include src="component.html"/>
<!-- component.html internally tries to access `title` from caller -->
```

Instead, make the bindings explicit.

### 🔒 Summary

* Like Rust’s hygienic macros, `include` **does not inherit host scope implicitly**.
* Values must be **declared via `bind:x="expression"`** at the call site.
* This avoids hidden dependencies, promotes reusability, and simplifies compiler reasoning.

---

## ⚙️ Evaluation Strategy

1. Evaluate `if` / `unless` guards.
2. Expand nodes using `iterate`, establishing a scope for each iteration.
3. Within each scope:

   * Apply `bind` assignments
   * Resolve `from` path to produce content or attribute value
   * If missing, try `fallback`
4. Output is compiled to a static, pre-evaluated document tree.
