This will be a high-performance library for handling the [FoLiA XML format](https://proycon.github.io/folia), a format for linguistic annotation.

At the moment, it is  still pre-alpha, under heavy development, and not functional enough yet.

## Implemented and (possibly) planned features

* [x] Full in memory representation
    * [x] Memory saving
* [ ] Streaming parser / partial representation
* [x] Features with subsets
* [ ] Editing/Mutability
    * (low-level implementation ready, higher level API needed)

* [x] Select mechanism

* [x] Structure annotation
* [x] Inline annotation
* [x] Span annotation
* [ ] Text markup
* [ ] Subtoken annotation (e.g. morphology)
* [ ] Alternative
* [ ] Corrections
* [ ] Substrings
* [ ] Relations

* [x] Native metadata
* [ ] Submetadata
* [ ] Foreign metadata
* [x] Provenance data
* [ ] Foreign annotations

* [ ] Shallow Validation
* [ ] Text consistency validation
* [ ] Deep validation

* [x] Text serialisation
* [x] XML serialisation
* [ ] JSON serialisation

* [ ] Folia Query Language (FQL)
    * (no real FQL parser yet but underlying selector and query provide a strong basis for implementation)
