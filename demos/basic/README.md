# Web Compiler – Basic Demo

This is a basic demonstration of the [Web Compiler](../../) — a custom static site compiler designed for developers who prefer authoring pure HTML fragments, compile-time components, and zero-runtime static sites.

Unlike typical static site generators, this compiler gives you full control over structure and semantics while still providing advanced features like layout composition, nested path resolution, and macro-like template expansion.

---

## 🗂 Project Structure

```
basic/
├── blog/          # HTML fragments for individual blog posts
├── common/        # Site-specific shared components (e.g. header, footer)
├── components/    # General-purpose reusable components (e.g. TabView)
├── pages/         # Main site pages (index, about, docs)
├── styling/       # CSS files (base styles + typography)
├── templates/     # Layout templates (global and blog-specific)
├── scripts/       # Helper scripts for local preview
└── web-compiler.toml  # Project config for compilation
```

---

## 🚀 Features Demonstrated

* Fragment-based HTML pages with `<include>` layout injection
* Reusable components with compile-time bindings and iteration
* Template inheritance (`global.html`, `blog-post.html`)
* Correct relative path resolution for nested page output
* Dark mode styling via `prefers-color-scheme`
* Static build output — no client-side JavaScript required

---

## 🛠 Running the Demo

From the root of the repo:

```bash
cargo run --bin web-compiler run --manifest demos/basic/web-compiler.toml
```

You may find these convenience scripts helpful as well (can be invoked from any directory):
> ```bash
> # Serve it locally (requires `http-server`)
> ./demos/basic/scripts/serve.sh
> ./demos/basic/scripts/open.sh
> ```

---

## 📄 License

This demo is provided under the same license as the main project. See the [root README](../../README.md) for more information.
