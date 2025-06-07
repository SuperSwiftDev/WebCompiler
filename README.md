# Web Compiler

A static-site compiler that treats HTML like code — composable, inspectable, and compiled with full semantic context.

**Web Compiler** is designed for developers, document authors, and small digital teams who want full control over their markup without the rigidity of CMSs, the noise of Markdown-based SSGs, or the overhead of JS frameworks.

This is the spiritual successor to [Subscript HTML](https://github.com/subscript-publishing/subscript-html), originally built for publishing academic notes with embedded math, graphs, and textbook-style structure — now re-imagined as a general-purpose static site compiler.

---

## ✨ Highlights

* **HTML-first** — All source content is written in pure HTML, with semantic, maintainable structure.
* **Recursive `<include>` support** — Compose templates naturally; path resolution is stable and automatic, even across deep nesting.
* **Compile-time macros** — Use tags like `<bind>`, `<enumerate>`, `<value>`, and `<content>` to define reusable components with no runtime cost.
* **Template wrapping** — Global templates like `base.html` or `blog-post.html` are applied automatically, with content projection via `<content>`.
* **Bottom-up evaluation** — The compiler processes content fragments before wrapping them, ensuring accurate link resolution, TOC generation, and macro expansion.

---

## 📁 Repository Layout

```
.
├── web-compiler/           # Binary entrypoint for compilation
├── web-compiler-core/      # Transformation engine (pre/post processing, macro logic)
├── web-compiler-common/    # Shared utilities (symlinks, path resolution, vpath encoding)
├── web-compiler-html-ast/  # HTML parsing, stringification, traversal engine
├── demos/                  # Example projects (basic/, advance/)
├── schema/                 # JSON schemas for macro tags (for editor support)
├── scripts/                # CLI, test, and formatting utilities
└── Cargo.toml              # Top-level workspace manifest
```

---

## 🧪 Getting Started

Run the basic demo:

```bash
cd demos/basic
cargo run --bin web-compiler run
./scripts/serve.sh
./scripts/open.sh
```

This builds the site using `web-compiler.toml`, wrapping fragments in a shared layout and compiling the output to `/output`.

Then open `http://127.0.0.1:8001` in your browser.

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

* [`subscript-html`](https://github.com/subscript-publishing/subscript-html) — compiler-based HTML preprocessing for educational publishing
* [`colbyn/school-notes`](https://github.com/colbyn/school-notes) — LaTeX-style layout system for STEM documentation
* Early frustrations with PostHTML + Parcel, which lacked bottom-up macro evaluation and consistent path resolution

Web Compiler improves on these by offering:

* A clean, stable AST
* Custom macro expansion passes
* Compile-time template injection
* Resolved link correctness by design

---

## 📄 License

MIT License. See [LICENSE](./LICENSE).
