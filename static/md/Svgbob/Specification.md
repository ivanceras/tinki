# Svgbob Specification

#### Characters
Svgbob relies on set of characters that are easily accessible on your keypad.
Each of these character has certain set of behaviors depending on the neighbor character.

These are the most common characters that are used.
```csv
Character, Common Name
., period or dot
",", comma
`, backtick or backquote
', quote or singlequote
+,plus sign or cross
*,asterisk
o, letter o
O, capital letter O
-, dash or hypen
|, vertical bar
~, tilde 
_, underscore
:, colon
!, exclamation mark
<, less than sign
>, greater than sign
v, letter v
V, capital letter V
^, caret
"""",Double quote



```


## Character general behaviors

#### Period `.`
 - Period makes a rounded corner on the top-left if a line connects to it from the bottom
and from the right.
```bob
  .--
  | 
```
 - Period makes a rounded corner on the top-right if a line connects to it from the bottom 
 and from the left.
```bob
  --.
    |
```


#### Comma `,`
Comma makes a rounded corner on the top-left if a line connects to it from the bottom
and from the right.
```bob
  ,--
  |
```


#### Backtick `` ` ``
Backtick makes a rounded corner on the bottom-left if a line connects to it from the top
and from the right.
```bob
  |
  `--
```

#### Single quote `'`
 - Single quote makes a rounded corner on the bottom-left if a line connects to it from the top
 and from the right.
```bob
  |   
  '--
```
 - Single quote makes a rounded corner on the bottom-right if a line connects to it from the top
 and from the left.
```bob
      |
    --'
```

#### Plus sign `+`
 - Plus sign makes a sharp corner when connected from 2 perpendicular lines.
```bob
   +--  --+
   |      |

   |      |
   +--  --+
```

 - Plus sign makes a cross-section when connected from 4 directions (top, right, bottom, left)
```bob
       |
     --+--
       |
```

#### Asterisk `*`
Asterisk makes small solid circle when connected to a line
```bob
  *---  ---*
```

#### Small letter `o`
The letter **o** makes small clear circle when connected to a line
```bob
  o---  ---o
```

#### Big letter `O`
The big letter **O** makes bigger clear circle when connected to a line
```bob
   O--- ---O---
```

#### Dash `-`
Dash makes a solid line. Place them next to each other to form a longer line
```bob
   ---------------
```

#### Broken line `- - -`
3 dash line with space in-between them (ie: `- - -`) makes a broken line
```bob
   - - - - - - - - - - -
```

####  Vertical bar `|` 
Vertical bar makes a vertical line
```bob
    |
    |
```

#### Tilde `~` 
Tilde makes a broken line
```bob
   ~~~~~~~~~~~~~
```

#### Underscore `_`
Underscore makes lowered solid line
```bob
   ________________
```

#### Colon `:`
Colon makes vertical broken line
```bob
    :
    :
```

#### Exclamation mark `!`
Exclamation mark makes a vertical broken line
```bob
    !
    !
```

#### Less than sign `<`
Less than sign makes an arrow to the left if a line connects to it from the right
```bob
   <-----
```

#### Greater than sign `>`
Greather than sign makes an arrow to the right if a line connects to it from left
```bob
   -----> 
```

#### Letter `V` (both lowercase and capital)
 - Letter `V` makes an arrow pointing bottom if a line connects to it from the top
```bob
    |
    V
```
 - Letter `V` makes an arrow pointing bottom-left if a line connects to it from the top-right
```bob
      /
     V
```
 - Letter `V` makes an arrow pointing bottom-right if a line connects to it from the top-left
```bob
    \
     V
```

#### Caret `^`
 - makes an upward arrow if a line connects to it from the bottom 
```bob
   ^
   |
```
 - makes an arrow pointing top-left if a line connects to it from the bottom-right
```bob
  ^
   \
```
 - makes an arrow pointing top-right if a line connects to it from the bottom-left
```bob
     ^
    /
```

[Back to Svgbob](../Svgbob.md)
