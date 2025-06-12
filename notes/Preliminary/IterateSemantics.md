## 🔁 `iterate="services"` – Semantics Defined

### 📖 Definition (Plain English):

> For this node, **repeat its child structure once per item** in the collection named `services`, binding each item to the local context for use by `from`, `prompt`, or `bind` attributes inside.

---

## 🧠 Formal Semantics (Step-by-Step)

Given:

```xml
<section iterate="services">
  <card>
    <h2 from="name"/>
    <p prompt="describe this service"/>
  </card>
</section>
```

Assume input context:

```json
{
  "services": [
    { "name": "Roof Repair", "description": "..." },
    { "name": "Window Replacement", "description": "..." }
  ]
}
```

**Execution Semantics:**

1. Lookup variable `services` in the current scope. Expect a list (e.g. array or iterable).
2. For each `item` in `services`:

   * Create a *new scope* where `.` (the default context) refers to `item`
   * Evaluate/expand the body (`<card>…</card>`) using that scope
   * Evaluate `from="name"` as `item.name`
   * If `prompt="..."`, treat it as LLM instruction using the new scope as the base
3. Replace the original `<section>` node with the **list of expansions** of its child nodes (one per item).

---

### ⚠️ Type Expectations

| Attribute | Expected Input                                              |
| --------- | ----------------------------------------------------------- |
| `iterate` | Named variable must be a **list/array**                     |
| `from`    | Any primitive or object path within current item            |
| `bind:x`  | Binds named variable into scope (can be loop counter, etc.) |

---

### 🧬 Extended Semantics: Binding Loop Variables

```xml
<section iterate="services" bind:index="loop.index">
  <card>
    <h2 from="name"/>
    <p prompt="describe service"/>
    <span from="index"/>
  </card>
</section>
```

Compiler behavior:

* `loop.index` is implicitly passed (0, 1, 2…)
* `bind:index="loop.index"` makes that accessible inside the loop body
* You can also expose:

  * `loop.first` → `true` for index 0
  * `loop.last` → `true` for final item
  * `loop.length` → total count

This allows formatting like:

```xml
<p if="loop.first">This is our most popular service!</p>
```

---

## 🛠 Internals You Might Model (Rust style pseudocode)

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

---

## Summary

| Attribute        | Purpose                                        |
| ---------------- | ---------------------------------------------- |
| `iterate="x"`    | Repeat the node for each item in `x`           |
| `from="..."`     | Lookup property on current item                |
| `bind:key="..."` | Define scoped variables                        |
| `if="..."`       | Conditional rendering based on current context |

This gives you *data-driven structure synthesis*, crucial for bridging structured inputs (like scraped data or form submissions) into generated UI.
