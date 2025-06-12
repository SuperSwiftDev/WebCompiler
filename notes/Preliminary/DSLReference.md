## 📘 **PageML DSL Reference: Core Semantics Cheat Sheet**

---

### 🧱 Core Node Types

| Element                               | Description                                                          |
| ------------------------------------- | -------------------------------------------------------------------- |
| `<page>`                              | Root document node. Attributes: `id`, `route`, `layout`              |
| `<region>`                            | Named content slot (e.g. `main`, `sidebar`)                          |
| `<section>`                           | Structural grouping of content                                       |
| `<card>`, `<hero>`, `<form>`, etc.    | Domain-specific layout components                                    |
| `<h1>`-`<h6>`, `<p>`, `<image>`, etc. | Semantic content leaves                                              |
| `<include src="..."/>`                | Reusable fragment import                                             |
| `<macro>`, `<use-macro>`              | Declarative component definition and invocation (optional extension) |

---

### 🔁 `iterate`

| Syntax        | `iterate="services"`                                                 |
| ------------- | -------------------------------------------------------------------- |
| Input         | A list of items from context                                         |
| Behavior      | Repeat this node once per item in the list, binding each item as `.` |
| Scope         | New child context with local bindings                                |
| Implicit vars | `loop.index`, `loop.first`, `loop.last`, `loop.length`               |
| Optional      | Use `bind:` to assign implicit vars explicitly                       |

```xml
<section iterate="services">
  <card>
    <h2 from="name"/>
    <p from="description"/>
  </card>
</section>
```

---

### 🧩 `from`

| Syntax   | `from="property.name"`                                               |
| -------- | -------------------------------------------------------------------- |
| Input    | String key or dotted path                                            |
| Behavior | Resolve property from current context or bound variable              |
| Errors   | Should fail loudly if key doesn’t exist unless fallback is specified |

```xml
<h2 from="title"/>
```

---

### 🎯 `prompt`

| Syntax           | `prompt="instruction to LLM"`                           |
| ---------------- | ------------------------------------------------------- |
| Behavior         | Marks node for generation by LLM using context + prompt |
| Context          | Includes all in-scope bindings (e.g. current loop item) |
| Rendered content | LLM output replaces inner content                       |

```xml
<p prompt="summarize this service in one sentence"/>
```

---

### ⚖️ `if` / `unless`

| Syntax   | `if="condition"` or `unless="condition"`                |
| -------- | ------------------------------------------------------- |
| Input    | Boolean expression (ideally variable lookup for now)    |
| Behavior | Conditionally includes or excludes the node             |
| Scope    | Uses current context (loop item, page-level data, etc.) |

```xml
<faq if="has_faqs">
  ...
</faq>
```

---

### 🧬 `bind:key`

| Syntax   | `bind:index="loop.index"`                              |
| -------- | ------------------------------------------------------ |
| Behavior | Binds an expression to a named variable in child scope |
| Use case | Loop counters, extracted values, temporary renames     |

```xml
<section iterate="services" bind:index="loop.index">
  <span from="index"/>
</section>
```

---

### 📦 `include`

| Syntax   | `<include src="components/footer.xml"/>`           |
| -------- | -------------------------------------------------- |
| Behavior | Load external XML fragment at compile time         |
| Use case | Reusable shared blocks (header, CTA, footer, etc.) |
| Note     | Path is resolved relative to current document      |

---

### 🧠 Default Context (`.`)

In a loop or fragment, `.` refers to the current object in context:

```xml
<p from="."/> <!-- means: use the string value of current item -->
```

---

## 🛠 Compiler Behaviors Summary

| Feature     | Compiler Behavior                                         |
| ----------- | --------------------------------------------------------- |
| `iterate`   | Walk list, rebind scope, recursively evaluate child nodes |
| `from`      | Resolve variable path in current scope                    |
| `prompt`    | Call out to LLM with current context and prompt string    |
| `if/unless` | Conditionally include/exclude node                        |
| `bind:`     | Extend current scope with new variable                    |
| `include`   | Inline external XML at parse time                         |

---

## ✅ Example Snippet

```xml
<page id="home" route="/" layout="default">
  <region name="main">
    <hero>
      <h1 prompt="concise headline about company"/>
      <p prompt="summarize what you offer"/>
    </hero>

    <section id="services" iterate="services" bind:index="loop.index">
      <card>
        <h2 from="name"/>
        <p from="description"/>
        <span from="index"/>
      </card>
    </section>

    <quote-list if="testimonials">
      <quote iterate="testimonials">
        <text from="text"/>
        <author from="author"/>
      </quote>
    </quote-list>
  </region>
</page>
```


```rust
struct EvaluationContext {
    variables: HashMap<String, Value>,
    parent: Option<Box<EvaluationContext>>,
}

fn evaluate_iterate_node(node: XmlNode, ctx: &EvaluationContext) -> Vec<XmlNode> {
    let iterable_name = node.get_attribute("iterate");
    let items = ctx.get_list(&iterable_name).expect("Must be a list");

    items.enumerate().map(|(i, item)| {
        let mut subctx = ctx.fork();  // New scoped context
        subctx.set(".", item);        // Local scope is the item
        subctx.set("loop.index", i);
        subctx.set("loop.first", i == 0);
        subctx.set("loop.last", i == items.len() - 1);
        subctx.set("loop.length", items.len());

        evaluate_node_children(node.children, &subctx)
    }).flatten().collect()
}
```

