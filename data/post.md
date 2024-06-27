

This post is an outgrowth from two previous threads, one on avoiding [pub use](https://users.rust-lang.org/t/avoid-pub-use/112156?u=crumplecup), well summed up by:

[quote="kaj, post:25, topic:112156"]
Do not `pub use` any item from a `pub mod`.

So I do either:

```
mod some_impl;
pub use some_impl:SomeType;
```

or

```
pub mod some_impl; // contains SomeType
```

but never:

```
pub mod some_impl;
pub use some_impl::SomeType
```
[/quote]

The second thread is on [preludes](https://users.rust-lang.org/t/to-use-prelude-or-to-not-to-use-prelude-that-is-the-question/110855?u=crumplecup), where several users echoed similar sentiments on having a single source of truth for a given type:

[quote="duelafn, post:3, topic:110855"]
I don't like the risk of random conflicts.
[/quote]

[quote="kpreid, post:4, topic:110855"]
I think it's good for a library to export items from **exactly one path each**, to reduce the number of arbitrary choices users of the library need to make. Adding a “prelude” creates a second path for the items.
[/quote]

[quote="kajacx, post:16, topic:110855"]
my rule of thumb when it comes to glob imports in general is to have at most one glob import (prelude or otherwise)
[/quote]


