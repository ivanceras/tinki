# Spongedown

Spongedown extends markdown with support for additional useful features.
 - svgbob diagrams
 - inline csv
 - embedding content


#### Svgbob diagram
[Svgbob](Svgbob.md) makes use of textual representation of diagram and creates and svg image out of it.

This is an example of svgbob diagram

```bob

                                                   +-----+------+
                                             .---> |-----|------|
                                            /      |-----|------|
                                           /       +-----+------+
                                          /                 .--.
                                         /                  |  |
                                        /                   v  |
  .-------.                            /           .-. .-. .-. |
  | Table |-.                         /        .-->'-' '-' '-' |
  '-------'  \                       / .-----> |     \  |  /   |
              \                     / /        |      v . v    |
.------------. \                   / /         |_______/ \_____|
| Flowcharts |--.                 / /                  \ /
'------------'   \               / /                    |     ____
                  v _______     / /                     '--> /___/
.--------.         /       \---' /
| Graphs |------->/ Sponge  \---'-.
'--------'     .->\  down   /----. \           ^  .  /\  .-.
              / .->\_______/-.    \ \          |_/ \/  \/   \
.--------.   / /              \    \ `-------> +------------->
| Comics |--' /                \    \
'--------'   /                  \    \         +------------+
            /                    \    \        |   .-----.  |
   .----------.                   \    \       |  (       ) +------------+
   | Diagrams |                    \    \      |   `-, .-'  |  .-----.   |
   '----------'                     \    `---> |    /,'     | (       )  |
                                     \         |   /'       |  `-. .-'   |
                                      \        |            |     `.\    |
                                       \       | ٩(̾●̮̮̃ ̾•̃̾)۶    |       `\   |
                                        \      |            |            |
                                         \     +------------|   (,⊙–⊙,)७ |
                                          `--.              +------------+
                                              \
                                               v           .-,(  ),-.
                                            ___  _      .-(          )-.
                                           [___]|=| -->(                )      __________
                                           /::/ |_|     '-(          ).-' --->[_...__...°]
                                                           '-.( ).-'
                                                                   \      ____   __
                                                                    '--->|    | |==|
                                                                         |____| |  |
                                                                         /::::/ |__|

```

#### CSV

CSV data are rendered as tables


    ```csv
    foo,bar,baz
    apple,banana,carrots
    rust,haskel,c
    1,2,3
    ```



```csv
foo,bar,baz
apple,banana,carrots
rust,haskel,c
1,2,3
```

#### Embedding content

```csv capture as input1
what,ever,goes,here
will,be,captured,and
can,be,referenced,later
on,
```

![```csv][input1]

#### Markdown works as is.

[Render and Example markdown](Markdown-example.md)


#### Links
 - [Code Repository](https://github.com/ivanceras/spongedown)
