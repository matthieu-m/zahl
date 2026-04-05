A pragmatic library of type-level integers

#   Status

This is an early project, the API is unstable, the whole thing may yet be scratched, use at your own risks.


#   Goals

##  Goals

-   **Human-friendly**: human-friendly diagnostics, human-readable symbols.


##  Non-Goals

-   **Completeness**: no pretension in being the end-all, be-all. See alternatives.
-   **Large Integers**: focus on small integers, which are sufficient for many usecases.


#   Alternatives

##  Why not typenum?

If you want a full-featured type-level integer library, you want typenum.

Unfortunately, the flexibility of typenum comes by encoding the numbers as their binary representation in the type
system, for example `U9` is a type synonym for `UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>`. This is great for
flexibility, it's atrocious when the compiler, or debugger, displays it, however.

Zahl aims for a more modest scope -- a much smaller set of integers -- and instead brings human-friendliness to the
table. Or tries to.


#   Name

In mathematics, the set of integers is named Z is derived from the german Zahl (number). Hence zahl seemed fitting for a
library wishing to model Z in the type system.
