# PageML Design Specification

## ✨ Design Abstract

**PageML** is a declarative, XML-based modeling language designed for the structural synthesis and content generation of web documents. It serves as a machine-readable intermediate representation (IR) that bridges high-level intent (natural language or structured data) and rendered web experiences (HTML, Markdown, JSON, etc.).

PageML empowers developers and LLMs to collaboratively design, populate, and render rich content layouts through a system of structural components, control attributes, and semantic hints. Unlike traditional HTML or templating systems, PageML elevates the abstraction to the level of *intent and structure*, allowing LLMs to infer content bottom-up while developers define modular and reusable layouts.

---

## 🔧 Technical Objectives

1. **Structural Modeling**: Define an expressive yet minimal syntax to represent pages, sections, components, and layout blocks.
2. **Declarative Generation Cues**: Allow semantic hints (e.g., `prompt`) to guide LLM-based content population.
3. **Data Integration**: Support `from` bindings for structured data and `iterate` for dynamic list expansion.
4. **Execution Semantics**: Enable bottom-up evaluation, with clearly defined scoping and loop mechanics.
5. **Extensibility**: Support domain-specific macros/components, reusable fragments, and layout templates.
6. **Compiler-Friendly Design**: Allow easy parsing and transformation via ASTs for static site generation or live rendering.

---

## 📚 Implementation Outline

The PageML system is composed of:

* **Input Language**: XML-based PageML files containing page, layout, and macro declarations.
* **Structured Data**: Optional JSON/YAML provided at compile-time as context.
* **Compiler Engine**: A recursive interpreter (e.g., written in Rust) that:

  * Resolves `iterate`, `from`, and `prompt` attributes
  * Applies macros and layout definitions
  * Outputs final content (HTML/JSON/etc.)
* **LLM Interface**: Pluggable pipeline that invokes LLMs to fulfill `prompt`-tagged nodes during compile-time or authoring.

---

## ✨ Core Syntax Overview

### Page Structure

```xml
<site>
  <page id="home" route="/" layout="default">
    <region name="main">
      ...
    </region>
  </page>
</site>
```

### Common Elements

```xml
<card>
  <h2 from="name"/>
  <p prompt="describe this item"/>
</card>
```

### Control Attributes

| Attribute       | Purpose                               |
| --------------- | ------------------------------------- |
| `from`          | Bind to structured data               |
| `prompt`        | Request LLM-generated content         |
| `iterate`       | Expand children for each item in list |
| `bind:x="..."`  | Bind local variables into scope       |
| `if` / `unless` | Conditional rendering                 |

### Macros and Includes

```xml
<macro name="cta">
  <h2 prompt="headline"/>
  <p prompt="supporting copy"/>
</macro>

<use-macro name="cta"/>
<include src="components/header.xml"/>
```

---

## 📊 Component Semantics

### 1. `<page>`

* **Attributes**: `id`, `route`, `layout`
* **Role**: Top-level document container.
* **Behavior**: Optional layout applied; regions named and populated.

### 2. `<region>`

* **Attributes**: `name`, `if`, `unless`
* **Role**: Matches layout slots; optional conditional rendering.

### 3. `<section>` / `<card>` / `<quote-list>` / custom components

* **Attributes**: Optional `iterate`, `if`, `bind`
* **Role**: Structural blocks for visual and semantic grouping.
* **Semantics**:

  * `iterate="services"` → lookup `services` in context → for each item:

    * Bind `.` to current item
    * Bind `loop.index`, `loop.first`, etc.
    * Evaluate and repeat child nodes

### 4. Content Nodes (`<h1>`, `<p>`, `<image>`, etc.)

* **Attributes**:

  * `from="key"` → pull data directly from current context
  * `prompt="describe ..."` → delegate content generation to LLM

### 5. `<macro>` / `<use-macro>`

* **Role**: Define reusable content structures.
* **Semantics**:

  * A macro is a named, parameterized content tree.
  * A `use-macro` can optionally pass variables via `bind:x="..."`

---

## ⚖️ Execution Model

### Evaluation Order (Bottom-Up Pass):

1. **Leaf nodes** evaluated first (from content or `prompt`)
2. **Iterative containers** (`iterate`) unfold children with scoped bindings
3. **Conditional nodes** skipped if guard fails
4. **Macros/includes** expanded
5. **Final page/region structure** assembled and returned

### Contextual Bindings:

* `.` → current data item (loop or page)
* `loop.index`, `loop.first`, `loop.last`, `loop.length`
* `bind:x` → create scoped variables

---

## 🚀 Future Directions

* Schema validation via Relax NG or XSD
* Authoring tools with visual layout + live LLM preview
* Versioned layout libraries (e.g., `layout="hero-v2"`)
* Component marketplaces for sharing macros and layouts
* Support for multi-modal content (e.g., `image.prompt`, video embeds)

