## 🧠 Conceptual Foundation

Think of your markup model as sitting at the level of:

> **Structural composition, semantic intent, and generation constraints.**

The simplest version (like your `<section>`, `<p>`, etc.) is just a *projection* of this core. But you want to support:

* **Deep composability** (like nested layout primitives, widgets, reusable fragments)
* **Semantic roles** (what this block *means*, not just what it looks like)
* **Domain-specific abstractions** (like `<quote-list>` or `<tabview>`)
* **Generation logic** (declarative prompts, bindings, iteration, etc.)

So let's define a richer *structural vocabulary*.

---

## 🏗️ Proposed Hierarchy of Structural Components

This is a sketch of how you might model your content stack in tiers:

### 1. **Root Document Types**

```xml
<site>
  <page id="services" route="/services">
    ...
  </page>
</site>
```

* `site`: can hold shared data, layout templates, etc.
* `page`: a named route with content and layout

---

### 2. **Regions / Layouts**

```xml
<page layout="default">
  <region name="main">
    ...
  </region>
  <region name="sidebar" if="has_sidebar">
    ...
  </region>
</page>
```

* `region`: container with a semantic role (not just a visual one)
* Supports override/slot-like behavior (`layout.html` might bind to `main`, `footer`, etc.)

---

### 3. **Blocks and Components**

```xml
<card title="Affordable Roofing">
  <body prompt="summarize key value"/>
  <icon src="roof.png"/>
</card>

<quote-list iterate="testimonials">
  <quote from="text"/>
  <author from="author"/>
</quote-list>
```

* Domain-specific elements: `<card>`, `<faq-list>`, `<step-list>`, `<testimonials>`
* These act like *macros or higher-order components*
* Expandable via your compiler with template definitions

---

### 4. **Fragments / Includes / Mixins**

```xml
<include src="components/header.xml"/>
<include src="fragments/roofing-intro.xml"/>
```

* Like partials, reusable content structures

Or define inline macros:

```xml
<macro name="cta-block">
  <h2 prompt="headline"/>
  <p prompt="supporting copy"/>
</macro>
```

Then reuse:

```xml
<use-macro name="cta-block"/>
```

---

### 5. **Content Elements (Leaf Nodes)**

```xml
<h1 prompt="main headline"/>
<p from="summary"/>
<image src="hero.jpg"/>
<list iterate="features">
  <item prompt="describe feature"/>
</list>
```

These are the lowest units passed to LLMs or rendered directly.

---

## ⚙️ Attributes & Control Constructs

To make it composable, you'll want:

| Attribute       | Purpose                                                 |
| --------------- | ------------------------------------------------------- |
| `prompt`        | Ask LLM to generate content from context or description |
| `from`          | Pull from named variable/data                           |
| `iterate`       | Expand a structure multiple times                       |
| `if` / `unless` | Conditionally render node                               |
| `bind:key`      | Assign a value to local variable                        |
| `role`          | Hint for tone/persona                                   |
| `slot`          | Name for projection into macro/component layout         |

Example:

```xml
<step-list iterate="steps" bind:index="loop.index">
  <step>
    <number from="index"/>
    <title from="step.title"/>
    <description prompt="elaborate on step meaning"/>
  </step>
</step-list>
```

---

## 🧬 Optional: Data Section

Embed structured data directly:

```xml
<data>
  <services>
    <item>
      <name>Roof Repair</name>
      <icon>roof.png</icon>
    </item>
    <item>
      <name>Window Replacement</name>
      <icon>window.png</icon>
    </item>
  </services>
</data>
```

Or support external injection (`--data input.json` during compile).

---

## 🛠 What This Enables

With this architecture, you can eventually:

* Compile to **HTML, Markdown, JSON**, etc.
* Support **LLM-based content filling** via traversal
* Compose **arbitrarily deep nested content** and render in a responsive layout
* Reuse blocks (`macros`) across pages and domains
* Swap styles/layouts without touching core content structure
