## 🧱 Base Format: `PageML` (Working Title)

```xml
<page title="Home" route="/">
  <section id="hero">
    <h1 prompt="summarize purpose of business"/>
    <p prompt="who is this for? what do they need?"/>
    <image src="logo.jpg" alt="Brand Logo"/>
  </section>

  <section id="services" iterate="services_list">
    <card>
      <h2 from="service.name"/>
      <p prompt="describe this service clearly and persuasively"/>
    </card>
  </section>

  <section id="about" if="has_about_section">
    <h2>About Us</h2>
    <p prompt="company origin story, mission, values"/>
  </section>
</page>
```

This format is not just structure—it’s a **generation scaffold.**

---

## 🧠 Core Concepts

| Element / Attribute          | Purpose                                                   |
| ---------------------------- | --------------------------------------------------------- |
| `<page>`                     | Top-level unit of the site (title, route, meta)           |
| `<section>`                  | Logical subdivisions, reused across pages                 |
| `<h1>`, `<p>`, `<img>`, etc. | Semantic primitives with LLM hints or direct values       |
| `prompt="..."`               | Signals LLM to *generate content* from scratch or summary |
| `from="..."`                 | Pulls from structured data or passed context              |
| `iterate="..."`              | Expands repeated structure (like cards, FAQs)             |
| `if="..."` / `unless="..."`  | Conditional presence (for content variation)              |
| `<include src="..."/>`       | Reference to external fragment (for reuse)                |

---

## ✳️ Meta-Model: XSD-style Schema in English

You could eventually enforce or document a structure like:

```plaintext
<page>
  - title (attribute)
  - route (attribute)
  - [0..∞] <section>
</page>

<section>
  - id (attribute)
  - iterate? (attribute) — list key
  - if? / unless? (attribute) — booleans
  - [0..∞] semantic elements (headings, paragraphs, images, etc.)
```

Semantic elements could follow HTML5-like types:

* `<h1>` through `<h6>`
* `<p>`, `<ul>`, `<li>`, `<image>`
* Custom wrappers like `<card>`, `<block>`, `<faq>`

---

## 🌀 Bottom-Up Traversal Strategy

A sample interpreter/compiler can do:

1. Traverse leaf elements:

   * If `from` → resolve via structured data
   * If `prompt` → send prompt and node to LLM
   * If text → leave as-is
2. Traverse containers:

   * If `iterate` → loop, clone subtree, pass context per iteration
   * If `if` / `unless` → evaluate context, skip if false
3. Assemble into static HTML (or virtual DOM, or JSON)

Optional: inline comments like this:

```xml
<!-- prompt="Make this sound confident, but concise" -->
```

These help make prompts explainable and auditable.

---

## 🧩 Future Extensibility

Eventually, you could add:

* **LLM roles/personas**:

  ```xml
  <p prompt="explain the process" role="friendly expert"/>
  ```

* **Tone or style modifiers**:

  ```xml
  <description tone="professional" prompt="summarize offering"/>
  ```

* **Fallbacks / defaults**:

  ```xml
  <testimonial prompt="generate if no real data" fallback="We were very pleased!"/>
  ```

* **Data sources** from JSON/CSV/YAML at the root of document or project.

---

## ✅ Example: Services Page

```xml
<page title="Services" route="/services">
  <section id="intro">
    <h1 prompt="write an inviting title for services page"/>
    <p prompt="briefly introduce what kinds of services are offered"/>
  </section>

  <section id="service-cards" iterate="services">
    <card>
      <h2 from="name"/>
      <p prompt="write a persuasive description of this service" from="description"/>
      <image from="icon"/>
    </card>
  </section>

  <section id="cta" if="should_include_cta">
    <h2>Ready to start?</h2>
    <p>Contact us today and get a free quote.</p>
  </section>
</page>
```

Where `services` is a list of objects like:

```json
[
  { "name": "Roof Replacement", "description": "...", "icon": "roof.png" },
  { "name": "Window Installation", "description": "...", "icon": "window.png" }
]
```
