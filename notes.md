Pretty much ranting that I can read to understand... what I don't understand?
Nothing really interesting unless you wanna read thoughts written on the top
of my head during my coding session (or rather just after, in general).

First impressions:
============================================================================
I believe the main problems with learning were:
- Still having difficulties grasping the ecosystem structure,
therefore never returning futures or polls at the right place.
If I had to draw a representation of how my code is currently
executed, it would probably take me about 10 mins (and it would
probably be partly wrong but that's not important for now #beginner).

I believe that the fact that it is so hard to hard a mental
picture of what is going on with my code isn't a good thing.

I don't think it comes from the structure of the framework because
after talking to a friend of mine, lot of the concepts and struct are
similar to libraries in other languages (he showed me some python code)
and those don't seem to suffer from this harsh learning curve.

I believe transparency on the internals mechanics of the library is
missing.
In Rust I'm much more used to understanding what it going with the
executing. By that I mean I can mentally picture an approximation of how
the code is going to be executed. I know that in reality the code will
get optimized, that some of the libraries method I call might be huge
but overall, none of these modification alter the flow of execution of
the code I write and none maintain hidden states (like a parallel
program executing along my code).
I know that the goal of tokio is to schedule my stuff therefore it is
necessary for it to do all of that and I don't really have anything
against it as long as I can picture what tokio is doing for me in my
back.
I mean for example I wrote quite a bunch of scripts in nodejs for work
and even though there's a magic scheduler doing all the callback calling
"in my back", it doesn't feel obscure (once you got how to write
asynchronous code of course).
I believe that this feeling arises from the fact the tokio is sharing a
bit more internal mechanisms than nodejs does, because it is looks like
it's been built to allow the building of other libraries that would make
it easy to use?

Let me explain.
In nodejs, you have a magic scheduler, but as a beginner
you don't even know that it exists. What you do know is that you have a
bunch of functions with callbacks that will do their job and call you
back. Simple. Send an HTTP request, then call my function with the
result.

In tokio, you see the scheduler because you start it (which is fine). The
first example you can read, shows you how to open a socket, accept
connections, write them a message and then close it.
That's fine as a hello world, but what I personally find frustrating with
this example is that it gives no room for easy customisation. By that I mean
even after further reading the tutorial, you can hardly make something
simple out of this code snippet. Like for example accept the socket, read
a message, write a response and close the socket.

I mean the next tutorial pages tell you about the execution model, the
future model, the ... Hey wait the tutorial changed! Well that task
stuff was interesting.

Well now my argument is getting nowhere because I'm lost.

What I believe would be great for the tokio learning experience would be
a basic string codec (sending a receiving strings) so that you can start
creating easily a client and a server exchanging strings. That would
remove a lot of the boiler plate codec stuff required to build up stuff
and help focus on the asynchronous programming I believe. (Or even a
generic structure codec based on basic serialization, that would be
awesome). If those ideas are already implemented, I might look like a
fool but that would be a great thing. They would just need to be
displayed in the tutorial for visibility.

What I believe is that a tutorial intended for beginners should be
articulated a bit like that:
- Learn asynchronous programing with closures only (no struct, no
complex function signature, no codec, no scheduler, just asynchronous
actions and their callbacks). This part should contain multiple
examples, more and more complex (but still readable of course).
- Then learn about codecs/streams to enrich and structure your data
flow.
- Then learn about the internal mechanics (futures, tasks, reactors, ...)
in a going deeper section.

The thing is that I believe that tokio lacks some tools to be easily
usable out of the box. But that might be fine. I mean maybe the goal of
the library is to lay the complicated foundation that will allow other
libraries to create easy to use/maybe less powerful APIs.

Too much talk not enough code. I believe I grasped something about that
tutorial thing that might be lacking and I'm gonna try to do something
about it.

Round 2:
============================================================================
Funny to read back all of that. Guess I was pretty tired that day ranting about
the framework structure. After writing that "dumb" example, it now seems pretty
simple to me.

But I believe I was right about the documentation. It really is a maze, and
understanding what the objects you are handling can cost you quite a
documentation pages to open. And that's when you already know where to find the
documentation of your object.

In fact, tokio::prelude is great. But used in the guide example, it hides the
module hierarchy. You then just never know where your object come from and
instead of being able to move in the library documentation to forge yourself
an understanding of its structure (what is where, what depends on what, what
was built on what), you just end up googling your way around.

I mean tokio::prelude is quite convenient, if you already know what you need
from the library and don't want to be bothered by cumbersome namespaces. But
I'm not too sure it is a great thing to include in the tutorials, unless the
tutorial somehow describes enough of the methods and types made available by
the prelude to make it useless to have to move through the library to build
basic applications (so that the user has the feeling he can build something
by himself when the tutorial's not there, and find the motivation to go
through the library when he needs more tooling for advanced stuff).

But yeah, as I said, I think I'm starting to grasp "how to" of the lib. Now I
think I'll just try and make a tons of small examples to learn about the
mechanisms around the futures. I think thoses examples will then allow me to
have basic usage examples of the types, trait and methods to serve as an
other source of documentation, hopefully less confusing than the docs.

Oh and I believe one of the reasons I got confused (in addition to the hard
to read docs) was that I tended to still think like I was programming in a
synchronous way (because I was in Rust maybe?). But thanks to a few days
of Typescript I'm back to asynchronous mode. Let's see where we can get from
there.

Ah and be fair I just found out there were line and bytes codec already
included in the library and that my ranting regarding that was quite unfair.
So I apologize, even though I still think these codecs should be exposed
in the tutorials, and showed used. I think that would be a better approach
than just explaining their implementation (that could be done afterwards).

And I might share those examples on github... Maybe.

Round 3:
============================================================================
I just found out why the tokio & future documentation seems so confusing to
me: it's organised as a tree/graph linking modules and crates in which you
navigate page by page.

This might sound quite dumb I guess: all of Rust documentation is formatted
exactly that way and I could use it all right. Yeah that's right, but to be
honest I never had to deal with any complex Rust library before tokio.

Before that, I always needed like just the methods of a single object or
the prototype of a single helper. All the associated types were standard, so
most of the time I never really had to navigate more than 1 or 2 pages of
documentation. And the documentation page format is pretty good. So it felt
like Rust documentation was perfect.

The documentation format (tree/graph) seems logical, because it follows the
struture of the code:
- Modules => tree
- extern/use => more generic graph (as opposed to a simple tree)
But truth is it give the documentation a "Wikipedia" feel: every time you
open a page of a structure of a complex library, you end up opening X pages.
And when the pages are all about traits and types you don't know... well
you're lost after 6-8 pages. And you just don't remember where you were
headed.

Plus the Rust trait system doesn't help much in this regard. I don't mean
that the trait system is bad, in fact I believe it is truly awesome. But
when an object implements a few traits, you end up with a s***load of
methods on the object, hidden in the trait unfoldable fields of the trait
section.
This leads to hardly being able to know the list of methods usable on a
structure because on the left navigation panel only the method specific
to the structure are shown. And unfolding the trait in the page leads to
a list of the traits methods with its full prototype specification and
a short description. That's great but it would be great to have an
equivalent unfolding trait field in the left panel that would simply list
the list of available method names and link them to the associated
description.

In comparison, in the C, lua or nodejs libraries I used until now the
documentation has, in general, been pretty flat. For example, there
would be a list of types and functions given by the library presented
as a flat list either on a single page or on a side panel. On more
complex libraries there could be 1 or max 2 levels of "module" levels
leading to more than one page, but the count seemed to stay reasonable.

I believe one of the reasons for this is that Rust libraries (tokio
in particular), tend to reuse a lot of trait and structs from other
libraries. That is of course a good thing but leads to the ecosystem
being more a lot of small modules importing each others than the
standard huge blob I got used to in other languages. Therefore it
feels like an understanding of what you are doing requires an
understanding of that whole dependency graph. And I believe it's
easy to get lost in that.

I got no solution for that. That was just my today's 2 cents on the
documentation :p
