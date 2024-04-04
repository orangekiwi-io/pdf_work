---

# Front Matter (YAML)

author: "Bob Wibble"
keywords: "PDF rust crates"
language: "en-GB"
title: "Untitled"
description: "A sample page."
wibble: "Wibble, a value untitled"

---

# My first markdown

The quick brown fox jumps over the lazy dog.

---

Example of replace yaml value (title): {{title}} ---

Example of replace yaml value (title) for a second time: {{title}} ---

Let us bring other YAML values into play:
* Wibble = {{wibble}}
* Language = {{lang uage}}
* Author = {{author}}

***

Example of yaml value not found (bob): {{bob}}
