Proof of concept to allow access to static memory without needing any heap 

TODO:
* Fetch an static by name and access it
* Provide types for the static memory items
* Don't use statics, instead use a macro in the main fn that creates the memory in the stack. Maybe this is not a good idea (Stack overflow)