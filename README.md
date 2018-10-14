tokio_examples
============================================

Just trying to make a few tokio usage examples to learn how to use the library.
Feel free to correct me if I'm wrong via pull requests I guess? (<= not used to open source collaboration)

Conventions
============================================

namespaces
--------------------------------------------
In all of my examples, I'll try to avoid to "use" submodules. In fact, as these
examples' goal is to help the reader (and myself) understand how to use tokio and I
believe having the full name of each element (struct of methods) might help
understand the structure/hierarchy of the future/tokio ecosystem. And I think that
knowledge might help getting comfortable using the library.

I'll therefore avoid using the crates preludes.

Examples
============================================

dumb
--------------------------------------------
This example consists of a server printing client messages and a client sending a
message then quiting.

