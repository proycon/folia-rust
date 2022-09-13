[![Crate](https://img.shields.io/crates/v/folia.svg)](https://crates.io/crates/folia)
[![Docs](https://docs.rs/folia/badge.svg)](https://docs.rs/folia/)
[![Build Status](https://travis-ci.com/proycon/folia-rust.svg?branch=master)](https://travis-ci.com/proycon/folia-rust)
[![Language Machines Badge](http://applejack.science.ru.nl/lamabadge.php/folia-rust)](http://applejack.science.ru.nl/languagemachines/)
[![Project Status: Inactive – The project has reached a stable, usable state but is no longer being actively developed; support/maintenance will be provided as time allows.](https://www.repostatus.org/badges/latest/inactive.svg)](https://www.repostatus.org/#inactive)


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
let selector = folia::Selector::from_query(&doc, &query).expect("selector");

//Run the selector
for word in doc.select(selector, folia::Recursion::Always) {
    //print the ID and the text
    println!("{}\t{}",
        word.id().or(Some("No-ID")),
        word.text(&folia::TextParameters::default())
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

We can create a document from scratch, all new elements can be added using the high-level ``annotate()`` method:

```rust
let doc = folia::Document::new("example", folia::DocumentProperties::default()).expect("instantiating folia");
let root: ElementKey = 0; //root element always has key 0
//add a sentence, returns its key
let sentence = doc.annotate(root,
                    folia::ElementData::new(folia::ElementType::Sentence).
                    with_attrib(folia::Attribute::Id("s.1".to_string())) ).expect("Adding sentence");

doc.annotate(sentence,
             ElementData::new(ElementType::Word)
             .with_attrib(Attribute::Id("word.1".to_string()))
             .with_text("hello".to_string())
            ).expect("Adding word 1");

doc.annotate(sentence,
             ElementData::new(ElementType::Word)
             .with_attrib(Attribute::Id("word.2".to_string()))
             .with_text("world".to_string())
            ).expect("Adding word 2");

```

Let's add a named entity for the above two words:

```rust
doc.annotate(sentence,
             ElementData::new(ElementType::Entity)
             .with_attrib(Attribute::Set("adhoc".to_string()))
             .with_attrib(Attribute::Class("greeting".to_string()))
             .with_span(&[ "word.1", "word.2" ])
).expect("adding entity");
```

Note that this will work regardless of the first parameter (``sentence``), as the span is explicitly provided:
``annotate()`` will automatically find out where add the layer (if needed).


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

### Declarations

All annotation types need to be declared in FoLiA, but the library does that for you automatically as long as you don't
set ``DocumentProperties.autodeclare`` to ``false``. Explicit declarations are done using ``Document.declare()``. Here
is a simple set-less declaration:

```rust
doc.declare(folia::AnnotationType::SENTENCE, &None, &None, &None);
```

Here a more elaborate one:

```rust
doc.declare(folia::AnnotationType::POS, Some("https://somewhere/my/pos/set".to_string()), &None, &None);
```

### Provenance

FoLiA v2 comes with extensive provenance support, so this library implements that as well. You can associate an active
processor by setting it in ``folia::DocumentProperties``:

```rust
    let processor = Processor::new("test".to_string()).autofill();
    let doc =  Document::new("example", DocumentProperties::default().with_processor(processor)).expect("document");
```

Switching processors on-the-fly can be done with ``doc.active_processor(processor_key)``. Any declarations made after
activating a processor will automatically assign that processor.

## Benchmarks

As the primary goal of this library is to provide a high-performance library, we ran some limited benchmarks against the other more mature and more feature complete FoLiA libraries: [FoliaPy](https://github.com/proycon/foliapy), written in Python, and [libfolia](https://github.com/LanguageMachines/libfolia), written in C++.

Tested on a Intel(R) Core(TM) i7-4770K CPU @ 3.50GHz, Linux 5.3

**Note:** The folia-rust implementation does only a minimal validation whereas the others do a a complete shallow validation
on parsing, including also a text consistency validation.

### Benchmarks on a +-100MB FoLiA document

(``bosb002gide03_01.nederlab.folia.xml``)

#### Parse from file into a full memory representation (DOM)


| Implementation | CPU | Memory | Peak Memory |
|--------------- |-----|--------|-------------|
| foliapy v2.2.1 | 60.9 s | 2083 MB |  - |
| libfolia v2.3 | 14.7 s | 2656 MB | 2681 MB |
| folia-rust v0.0.1 | 2.6 s | 531 MB | 622 MB |

#### Selecting and iterating over all words

| Implementation | CPU | Memory | Peak Memory |
|--------------- |-----|--------|-------------|
| foliapy v2.2.1 | 1.46 s | - |  - |
| libfolia v2.3 | 0.84 s | - | - |
| folia-rust v0.0.1 | 0.122 s | - | - |

#### Serialisation (without disk writing)

| Implementation | CPU | Memory | Peak Memory |
|--------------- |-----|--------|-------------|
| foliapy v2.2.1 | 77.7 s | - |  - |
| libfolia v2.3 | 5.06s | - | - |
| folia-rust v0.0.1 | 1.14s | - | - |

