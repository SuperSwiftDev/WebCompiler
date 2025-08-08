# Web Compiler

A static-site compiler that treats HTML like code — composable, inspectable, and compiled with full semantic context.

**Web Compiler** is designed for developers who want full control over their static sites without the rigidity of CMSs, the noise of Markdown-based SSGs, or the overhead of JS frameworks.

This is the spiritual successor to [Subscript HTML](https://github.com/subscript-publishing/subscript-html), originally built for publishing academic notes with embedded math, graphs, and textbook-style structure — now re-imagined as a general-purpose static site compiler.

# Real World Example

My site at [colbyn.com](https://colbyn.com) is now build with `web-compiler` using GitHub pages!

You may see the [source code over here](https://github.com/colbyn/ColbynDotCom) for a real-world example setup.

---

## ✨ Highlights

* **HTML-first** — All source content is written in pure HTML, with semantic, maintainable structure.
* **Recursive `<include>` support** — Compose templates naturally; path resolution is stable and automatic, even across deep nesting.
* **Compile-time macros** — Use tags like `<bind>`, `<enumerate>`, `<value>`, and `<content>` to define reusable components with no runtime cost.
* **Template wrapping** — Global templates like `base.html` or `blog-post.html` are applied automatically, with content projection via `<content>`.
* **Bottom-up evaluation** — The compiler processes content fragments before wrapping them, ensuring accurate link resolution, TOC generation, and macro expansion.

---

## 🛠 Core Tags and Macros

Web Compiler uses custom HTML tags that act like **compile-time macros** — they look like regular HTML, but are expanded during the build process into static output.

### `<include>` – Layout Composition

The `<include>` tag lets you embed external HTML fragments into a file, recursively. It enables modular layouts without relying on a JavaScript runtime or templating engine.

#### Example

```html
<!-- pages/index.html -->
<include src="../common/main.html">
    <h1>Hello World</h1>
</include>

<!-- common/main.html -->
<include src="header.html"></include>
<main>
    <content></content>
</main>
<include src="footer.html"></include>
```

This builds into a complete HTML page with header, content, and footer stitched together.

#### Features

* ✅ Recursive nesting: includes can contain other includes
* ✅ Path-correct: relative links (`./img/logo.png`) work as expected
* ✅ Content projection: the inner body is injected at the `<content>` placeholder in the fragment
* ✅ Used in templates: you can wrap entire page trees using a layout template that itself includes components

---

Additional macros include:

* `<content>` — Content projection point (used inside includes/templates)
* `<bind>` — Assigns variables from attributes (used in components)
* `<enumerate>` — Iterates over child nodes
* `<value>` — Outputs the value of a bound variable

These macros are inspired by compiler design: **top-down scoping + bottom-up transformation**, which ensures correctness and composability without runtime JS.

---

## 🔍 Real-World Use Cases

* **Developer sites & documentation**
  Treat HTML like source code. Use templates and macros for component reuse, without learning a new syntax.

* **Academic publishing**
  Originally used to generate STEM-heavy notes with LaTeX-style math (`<equation>`), live graphs (`<desmos>`), and multi-column layouts (`<layout>`), all from HTML.

* **Contractor/local business sites**
  Current demos explore real-world sites with perfect Lighthouse scores, ideal for SEO-conscious businesses that need speed, clarity, and long-term maintainability.

---

## 🧬 Lineage

This project builds on lessons from:

* [`SuperSwiftSites`](https://github.com/SuperSwiftDev/SuperSwiftSites) the previous implementation which has been refactored, renamed, and now lives here as `web-compiler`.
  - ⚠️ Still working on feature parity with the previous version.
* [`subscript-html`](https://github.com/subscript-publishing/subscript-html) — the original compiler-based HTML preprocessing for educational publishing
  
  **Examples:**
  - [My Beautiful Math Notes](https://colbyn.github.io/school-notes-spring-2020/)
* Early frustrations with PostHTML + Parcel, which lacked bottom-up macro evaluation and consistent path resolution (cannot be understated)

Web Compiler improves on these by offering:

* A clean, stable AST
* Custom macro expansion passes
* Compile-time template injection
* Resolved link correctness by design

---

## Updates

### 2025-8-8

#### Heading Baselines

Added a new baseline feature that will automatically decrement headings relative to some given baseline.

For example:

- `./index.html`:
  ```html
  <h1>Timeline</h1>
  <h2>2019</h2>
  <include
      src="./milestone.html"
      baseline="h3">
  </include>
  ```

- `./milestone.html`:
  ```html
  <h1 class="title">Milestone</h1>
  <p>At the end of 2019 I raked up an impressive 1,323 contributions on GitHub that placed me among the platform’s most active developers.</p>
  ```

- > Compiling `./index.html` will result in the following,
  >
  > ```html
  > <h1>Timeline</h1>
  > <h2>2019</h2>
  > <h3 class="title">Milestone</h3>
  > <p>At the end of 2019 I raked up an impressive 1,323 contributions on GitHub that placed me among the platform’s most active developers.</p>
  > ```
