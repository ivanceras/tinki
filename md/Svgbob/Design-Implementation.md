# Design Implementation

Svgbob converts characters into a graphical element, however listing every possible combination of characters for each of the 8 neighbors
would be exhausting and impractical.

For each character, we will subdivide it into 25 blocks at 5x5 cells.

```bob

        ┌─┬─┬─┬─┬─┐
        │a│b│c│d│e│
        ├─┼─┼─┼─┼─┤
        │f│g│h│i│j│
        ├─┼─┼─┼─┼─┤
        │k│l│m│n│o│
        ├─┼─┼─┼─┼─┤
        │p│q│r│s│t│
        ├─┼─┼─┼─┼─┤
        │u│v│w│x│y│
        └─┴─┴─┴─┴─┘

```

So, a character like `-` it will be line connecting from block `k` to block `o`
