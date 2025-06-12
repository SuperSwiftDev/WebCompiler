# Context Frames in ContentML

This document describes how the `<context>` element is used in ContentML to guide LLM-driven content generation by appending to the current context stack.


## 📘 Overview

The `<context>` element provides **lexical context** for downstream `<provision>` elements. It allows you to define natural-language instructions or guiding phrases that influence how content is generated, without modifying the underlying HTML structure.

Each `<context>` element adds a new frame to the current **context stack**, which is evaluated top-down during the provisioning phase. These frames can either be defined inline or reference a **named context stack** from the global context registry using the `for` attribute.


## 🔧 Behavior

* Appends a `ContextFrame` to the current lexical stack.
* Affects all nested `<provision>` elements within the same scope.
* Stack frames are **evaluated top-down**, with outer frames appearing earlier in the prompt.
* `<context>` can operate in two modes:

  1. **Inline**: Defines a new frame from its inner text.
  2. **Reference**: Appends a named stack using the `for` attribute.


## 🧩 Examples

### 1. Inline Context Frame

```xml
<context>
  Write in a warm, conversational tone suitable for local homeowners.
</context>

<provision src="content/homepage/hero.txt">
  Hero blurb introducing the company and core value proposition.
</provision>
```

### 2. Referencing a Named Stack

```xml
<context for="homepage" />

<provision src="content/homepage/cta.txt">
  Short call to action encouraging visitors to sign up.
</provision>
```

In this case:

* The compiler pulls the `"homepage"` context stack from the registry.
* Its frames are appended to the current context stack.
* The `<context>` tag body is ignored when `for` is present.


## 🧠 Notes

* Multiple `<context>` elements can be used in sequence or nested form. All are included in the evaluated stack.
* When `for="..."` is present, the referenced stack is appended to the current stack.
* Inner content is only used in **inline mode** (i.e. when `for` is **not** present).
* Content should be written in natural language—just as you’d describe a writing assignment to a human copywriter.


## ✨ Summary

The `<context>` element allows you to shape the tone, intent, and specificity of LLM-generated content directly in markup. Whether defining a local hint or reusing a named profile, it enables modular, transparent control of how copy is framed across the page structure.
