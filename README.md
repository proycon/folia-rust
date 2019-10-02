[![Crate](https://img.shields.io/crates/v/folia.svg)](https://crates.io/crates/folia)
[![Docs](https://docs.rs/folia/badge.svg)](https://docs.rs/folia/)
[![Build Status](https://travis-ci.com/proycon/folia-rust.svg?branch=master)](https://travis-ci.com/proycon/folia-rust)
[![Language Machines Badge](http://applejack.science.ru.nl/lamabadge.php/folia-rust)](http://applejack.science.ru.nl/languagemachines/)

This is a high-performance Rust library for handling the [FoLiA XML format](https://proycon.github.io/folia), a rich format
for linguistic annotation.

This library is currently in alpha stage, it may already be used to read FoLiA documents and to create documents from
scratch. **Note that this library does not yet implement validation!**. You will have to ensure your FoLiA documents are
valid by running another FoLiA validator, as this library does not yet guarantee producing valid FoLiA.

For a comparison of FoLiA libraries and a list of implemented features, see [FoLiA Implementations](https://folia.readthedocs.io/en/latest/implementations.html).

## Installation

Add ``folia`` to your project's ``Cargo.toml``.

## Usage

Reading from file and querying all words:

```rust
extern crate folia;

use folia;

//load document from file
let doc = folia::Document::from_file(filename, folia::DocumentProperties::default()).expect("parsing folia");
//Build a query, here you can match on any attribute
let query = folia::Query::select().element(folia::Cmp::Is(folia::ElementType::Word));
//Turn the query into a specific selector
let selector = folia::Selector::from_query(&doc, &query);

//Run the selector
for word in doc.select(selector, folia::Recursion::Always) {
    //print the ID and the text
    println!("{}\t{}",
        word.id().or(Some("No-ID")),
        word.text(None,None,false,true)
    );
}
```

A common pattern is to query in two stages,  methods like ``get_annotation()``, ``get_annotations()`` provide shortcut
alternatives to ``select()``. Let's output Part-of-Speech tags:

```rust
//Run the selector
for word in doc.select(selector, folia::Recursion::Always) {
    if let Some(pos) = word.get_annotation(folia::AnnotationType::POS, folia::Cmp::Any, folia::Recursion::No) {
        println!(pos.class().unwrap());
    }
}
```

We can create a document from scratch:

```rust
let doc = folia::Document::new("example", folia::DocumentProperties::default()).expect("instantiating folia");
let root: ElementKey = 0; //root element always has key 0
//add a sentence, returns its key
let sentence = doc.add_element_to(root,
                    folia::ElementData::new(folia::ElementType::Sentence).
                    with_attrib(folia::Attribute::Id("s.1".to_string())) ).expect("Adding sentence");

doc.add_element_to(sentence,
                 folia::ElementData::new(folia::ElementType::Word).
                              with(folia::DataType::text("hello"))).expect("Adding word 1");
doc.add_element_to(sentence,
                 folia::ElementData::new(folia::ElementType::Word).
                              with(folia::DataType::text("world"))).expect("Adding word 2");

```


If you have an element's key (a numerical internal identifier), you can easily obtain a ``FoliaElement`` instance:

```rust
if let Some(element) = doc.get_element(key) {

}
```

If you have it's official ID, you can do:

```rust
if let Some(element) = doc.get_element_by_id("example.s.1.w.1") {

}
```
