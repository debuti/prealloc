Proof of concept to allow access to static memory as if it were a heap 

TODO:
* [x] Fetch an static by name and access it
* [x] Provide types for the static memory items
* [N] Don't use statics, instead use a macro in the main fn that creates the memory in the stack. Maybe this is not a good idea (Stack overflow)
* [] no_std feature flag
* [] tests (copy from paste crate)
* [] docs (copy from paste crate) and README.md
* [] impl a simple heapless test app with it 
* [] examples: complex type, heap depletion, send and sync
* [] publish crates.io