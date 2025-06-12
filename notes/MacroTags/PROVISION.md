# Provisioning LLM-Generated Content

This document describes the `<provision>` element in ContentML, which acts as a declarative hook for generating and embedding copywriting content via LLMs.

## 📘 Overview

The `<provision>` element specifies a placeholder within HTML that triggers **one-time content generation** during the provisioning phase. It uses the **current context stack** (inherited from surrounding `<context>` elements) along with a human-written **prompt hint** to generate copy for the site.

The result is saved to a dedicated file, ensuring future runs remain **idempotent**—the model is not re-invoked unless the output file is deleted or regenerated.

## 🔧 Behavior

* If the `src` file **does not exist**:

  * The system builds a prompt from the current context stack and the inline body of the `<provision>` element.
  * The LLM is invoked once.
  * The result is written to the specified file.
  * The content is injected at compile time.

* If the `src` file **does exist**:

  * The file is read directly.
  * The content is injected without re-invoking the LLM.

This ensures safe, modular provisioning that blends automation with long-term manual refinement.

## 🧩 Example

```html
<provision src="content/homepage/hero.txt">
  Hero blurb highlighting core value proposition for the business
</provision>
```

In this example:

* The inline body provides a short prompt "hint."
* The generated copy is saved to `content/homepage/hero.txt`.
* If the file exists, it will be reused as-is.

## 🧪 Attributes

| Attribute | Type     | Required | Description                                                                    |
| --------- | -------- | -------- | ------------------------------------------------------------------------------ |
| `src`     | `string` | ✅ Yes    | Path to the output file where generated content will be saved and reused.      |
| `context` | `string` | ❌ No     | Name of a context profile from the registry. Inherits from parent if omitted.  |
| `format`  | `string` | ❌ No     | Output format hint (e.g. `inline`, `markdown`, `html`). Guides prompt shaping. |

## 🧠 Notes

* The **inner content** of the `<provision>` tag acts as a localized prompt hint and is not included in the final HTML output.
* Provisioned files are never overwritten unless deleted manually—allowing manual editing after the initial LLM pass.
* To support editorial workflows or A/B testing, use `<provision-group>` (coming soon).

## ✨ Summary

The `<provision>` element provides:

* A declarative way to generate copy content with LLMs.
* Integration with contextual prompts via inherited `<context>` frames.
* Persistent outputs stored in versioned files.
* A streamlined way to prototype, publish, and refine page copy.

----

## 🧰 Common Usage Patterns

Below are practical examples showing how to use `<provision>` effectively in real-world scenarios.

### 🔹 1. Single Blurb Provisioning

Provision a short inline description for a key section of a homepage:

```html
<context>
  Write in a clear, energetic tone aimed at first-time homeowners.
</context>

<provision src="content/homepage/value-prop.txt">
  Concise value proposition explaining why this service is trusted locally.
</provision>
```

### 🔹 2. Shared Context Across Multiple Provisions

Apply one context frame to multiple content blocks:

```html
<context>
  All content below should appeal to working parents looking for reliable, affordable service.
</context>

<provision src="content/homepage/hero.txt">
  Hero section intro blurb.
</provision>

<provision src="content/homepage/testimonial.txt">
  Sample testimonial from a satisfied customer.
</provision>
```

### 🔹 3. Custom Format Hint

Use the `format` attribute to shape LLM output:

```html
<provision src="content/homepage/features.md" format="markdown">
  Bullet list of 3 key service features.
</provision>
```

### 🔹 4. Referencing a Named Context Stack

You can point to a predefined context profile from the registry:

```html
<provision src="content/about/story.txt" context="company-story">
  Narrative paragraph describing the founding of the company.
</provision>
```

### 🔹 5. Iterative Development via File Deletion

To regenerate content during development, simply delete the provisioned file:

```sh
rm content/homepage/hero.txt
```

On the next build, the compiler will re-invoke the LLM using the same prompt stack.

