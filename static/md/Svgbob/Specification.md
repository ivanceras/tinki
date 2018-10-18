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
<div class="side-to-side">
<div>

```none
  .--
  |
```
</div>
<div>

```bob
  .--
  |
```
</div>
</div>



 - Period makes a rounded corner on the top-right if a line connects to it from the bottom
 and from the left.
<div class="side-to-side">
<div>


```none
  --.
    |
```
</div>
<div>

```bob
  --.
    |
```
</div>
</div>



#### Comma `,`
Comma makes a rounded corner on the top-left if a line connects to it from the bottom
and from the right.
<div class="side-to-side">
<div>

```none
  ,--
  |
```
</div>
<div>

```bob
  ,--
  |
```
</div>
</div>



#### Backtick `` ` ``
Backtick makes a rounded corner on the bottom-left if a line connects to it from the top
and from the right.
<div class="side-to-side">
<div>

```none
  |
  `--
```
</div>
<div>

```bob
  |
  `--
```
</div>
</div>



#### Single quote `'`
 - Single quote makes a rounded corner on the bottom-left if a line connects to it from the top
 and from the right.
<div class="side-to-side">
<div>

```none
  |
  '--
```
</div>
<div>

```bob
  |
  '--
```
</div>
</div>



 - Single quote makes a rounded corner on the bottom-right if a line connects to it from the top
 and from the left.
<div class="side-to-side">
<div>

```none
      |
    --'
```
</div>
<div>

```bob
      |
    --'
```
</div>
</div>



#### Plus sign `+`
 - Plus sign makes a sharp corner when connected from 2 perpendicular lines.
<div class="side-to-side">
<div>

```none
   +--  --+
   |      |

   |      |
   +--  --+
```
</div>
<div>

```bob
   +--  --+
   |      |

   |      |
   +--  --+
```
</div>
</div>


 - Plus sign makes a cross-section when connected from 4 directions (top, right, bottom, left)
<div class="side-to-side">
<div>

```none
       |
     --+--
       |
```
</div>
<div>

```bob
       |
     --+--
       |
```
</div>
</div>



#### Asterisk `*`
Asterisk makes small solid circle when connected to a line
<div class="side-to-side">
<div>

```none
  *---  ---*
```
</div>
<div>

```bob
  *---  ---*
```
</div>
</div>



#### Small letter `o`
The letter **o** makes small clear circle when connected to a line
<div class="side-to-side">
<div>

```none
  o---  ---o
```
</div>
<div>

```bob
  o---  ---o
```
</div>
</div>



#### Big letter `O`
The big letter **O** makes bigger clear circle when connected to a line
<div class="side-to-side">
<div>

```none
   O--- ---O---
```
</div>
<div>

```bob
   O--- ---O---
```
</div>
</div>



#### Dash `-`
Dash makes a solid line. Place them next to each other to form a longer line
<div class="side-to-side">
<div>

```none
   ---------------
```
</div>
<div>


```bob
   ---------------
```
</div>
</div>



#### Broken line `- - -`
3 dash line with space in-between them (ie: `- - -`) makes a broken line
<div class="side-to-side">
<div>

```none
   - - - - - - - - - - -
```
</div>
<div>

```bob
   - - - - - - - - - - -
```
</div>
</div>



####  Vertical bar `|`
Vertical bar makes a vertical line
<div class="side-to-side">
<div>

```none
    |
    |
```
</div>
<div>

```bob
    |
    |
```
</div>
</div>



#### Tilde `~`
Tilde makes a broken line
<div class="side-to-side">
<div>

```none
   ~~~~~~~~~~~~~
```
</div>
<div>

```bob
   ~~~~~~~~~~~~~
```
</div>
</div>



#### Underscore `_`
Underscore makes lowered solid line
<div class="side-to-side">
<div>

```none
   ________________
```
</div>
<div>

```bob
   ________________
```
</div>
</div>



#### Colon `:`
Colon makes vertical broken line
<div class="side-to-side">
<div>

```none
    :
    :
```
</div>
<div>

```bob
    :
    :
```
</div>
</div>



#### Exclamation mark `!`
Exclamation mark makes a vertical broken line
<div class="side-to-side">
<div>

```none
    !
    !
```
</div>
<div>

```bob
    !
    !
```
</div>
</div>



#### Less than sign `<`
Less than sign makes an arrow to the left if a line connects to it from the right
<div class="side-to-side">
<div>

```none
   <-----
```
</div>
<div>

```bob
   <-----
```
</div>
</div>



#### Greater than sign `>`
Greather than sign makes an arrow to the right if a line connects to it from left
<div class="side-to-side">
<div>

```none
   ----->
```
</div>
<div>

```bob
   ----->
```
</div>
</div>



#### Letter `V` (both lowercase and capital)
 - Letter `V` makes an arrow pointing bottom if a line connects to it from the top
<div class="side-to-side">
<div>

```none
    |
    V
```
</div>
<div>


```bob
    |
    V
```
</div>
</div>



 - Letter `V` makes an arrow pointing bottom-left if a line connects to it from the top-right
<div class="side-to-side">
<div>

```none
      /
     V
```
</div>
<div>

```bob
      /
     V
```
</div>
</div>


 - Letter `V` makes an arrow pointing bottom-right if a line connects to it from the top-left
<div class="side-to-side">
<div>

```none
    \
     V
```
</div>
<div>


```bob
    \
     V
```
</div>
</div>



#### Caret `^`
 - makes an upward arrow if a line connects to it from the bottom
<div class="side-to-side">
<div>

```none
   ^
   |
```
</div>
<div>

```bob
   ^
   |
```
</div>
</div>


 - makes an arrow pointing top-left if a line connects to it from the bottom-right
<div class="side-to-side">
<div>

```none
  ^
   \
```
</div>
<div>

```bob
  ^
   \
```
</div>
</div>



 - makes an arrow pointing top-right if a line connects to it from the bottom-left
<div class="side-to-side">
<div>

```none
     ^
    /
```
</div>
<div>

```bob
     ^
    /
```
</div>
</div>

#### Double quotes

Double quotes is used as an escape to prevent svgbob from interpreting the characters as drawing character and use them as text instead.

<div class="side-to-side">
<div>

```none
".----------------."
"| Don't draw me  |"
"`----------------'"
```
</div>
<div>

```bob
".----------------."
"| Don't draw me  |"
"`----------------'"
```

</div>
</div>

[Back to Svgbob](../Svgbob.md)
