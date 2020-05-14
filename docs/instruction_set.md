# Instruction Set

## Interger instructions:

 * `inc<width> <op>`
 * `dec<width> <op>`
 * `add<width> <op1>, <op2>`
 * `sub<width> <op1>, <op2>`
 * `mul<width> <op1>, <op2>`
 * `div<width> <op1>, <op2>`
 * `mod<width> <op1>, <op2>`
 * `divMod<width> <op1>, <op2>, <op3>` - `<op1> := <op1> / <op3>, <op2> := <op1> mod <op3>`
 * `umul<width> <op1>, <op2>`
 * `udiv<width> <op1>, <op2>`
 * `umod<width> <op1>, <op2>`
 * `udivMod<width> <op1>, <op2>, <op3>`
 
So for example `add8 r1_8 r2_8` is the instruction to add the value for `r2_8` to the value stored in `r1_8` and it will be stored in the register `r1_8`.

The width can be ommited if the width can be determined by the with of the first operand. 
So `add r1_16, [r2]` would be valid and equal to `add r1_16, [r2]`, but `add [r2], r1_16` would not be valid since it can't be derived how wide first operand should be.

Valid value for `width` are `8, 16, 32, 64`.

## Logic

 * `neg<width> <op>` - negates a two-compilemts number with `width` bits
 * `not<width> <op>` - inverts each bit
 * `and<width> <op>` - a bitwise and-operation
 * `or<width> <op>` - a bitwise or-operation
 * `xor<width> <op>` - a bitwise xor-operation
 * `shl<width> <op>, <amount>` - performs a left shift of the operant by `amount` bits. While shifting the most sicnificant bit will be shifted into the `CF` and zeros will be shifted in. So if `r1_8 == 0b10100101, CF == ?` after `shl r1_8, 1` would hold `r1_8 == 0b01001010, CF == 1` and after `shl r1_8, 5` would hold `r1_8 == 0b10100000, CF = 0`
 * `shr<width> <op>, <amount>` - performs a right shift of the operant by `amount` bits. Symetrical to `shl`.
 
The `<amount>` can be an addressing or an register.
If `amount > width + 1` the operand `<op>` will always be zeros and the `CF` will also always be zero.
 
## Float instructions:

 * `fadd<width> <op>, <op>`
 * `fsub<width> <op>, <op>`
 * `fmul<width> <op>, <op>`
 * `fdiv<width> <op>, <op>`
 
 Valid values for `width` are `32, 64`.
 
 ## Load/Store instructions
 
  * `mov<width> <op1>, <op2>`

There is also `copy` instruction that can copy more than 8 byte of data around.

 * `copy <reg>, <addr>, <addr>`
 
The first operand holds the amount of bytes to copy, the second is the base address for the source and the third is the base address for the destination. This instruction might need as much cycels to as bytes to copy to complete.

 * `swap<width> <op1>, <op2>` - swaps the content of the two operands

## Stack operations

 * `pop<width> <reg>`
 * `popFlags`
 * `popAll`
 * `push <reg>`
 * `pushFlags`
 * `pushAll`
 
## Branching

 * `cmp<width> <op1>, <op2>` - determines a comparison between `<op1>` and `<op2>` both signed 2s complement numbers and sets the Flags `LF`, `GF` and `EF` accordingly
 * `ucmp<width> <op1>, <op2>` - determines a comparison between two unsigned intergers and sets the flags `LF`, `GF` and `EF` accordingly
 * `fcmp<width> <op1>, <op2>` - determines a comparison between two unsigned intergers and sets the flags `LF`, `GF` and `EF` accordingly
 
### Labels

A label in the code is something that identifies either a position in the code section or a location in the read only or readable static memory section. With the command `#include <path> as <id>` can also the label of another code file come into scope:
For example: the file `a.sasm` contains:
~~~
...
.code:
.startscope strlen:
    pushAll
    sub r1, r1
    loop_start:
    ...
.endscope:
...
~~~
we can address the label `strlen` in another code file `b.sasm` like:
~~~
#include <a.sasm> as A
...
.code:
...
    jmp A/strlen
...
~~~

Note that `.startscope <name>:` creates a label for that scope and that every label defined in that scope is only addressable by prefixing it with `name/`.
With `../` you can go up a scope, so `jmp ../something` would jump to the label `something:` defined in the current parent scope.
Also note that you a label defined not in the current scope can only be reach be traversing the scope tree.

You can also indentify a label using `/` as a prefix for the gloabel scope.
So the following would be correct:
~~~
...
.code:
   
.startscope A:
    pushAll
    cmp r1, r2
    jmpE equals
    ...
    jmp end:
    equals:
    ...
    end:
    popAll
    ret
.endscope:

.startscope B:
    pushAll
    ...
    jmp /A/equals
    jmp ../A/equals # this both would be the same
    ...
.endscope:
~~~
 
This can be used together with jumping instructions to create branching in the code:

### Jumping

 * `jmp <label>` - jumps to the specified label in the code
 * `jmpL <label>` - jumps to the label if the `LF` flag is set
 * `jmpLE <label>` - jumps to the label if the `LF` flag or the `EF` is set
 * `jmpE <label>` - jumps to the label if the `EF` flag is set
 * `jmpGE <label>` - jumps to the label if the `GF` flag or the `EF` flag is set
 * `jmpG <label>` - jumps to the label if the `GF` flag is set
 * `jmpNL <label>` - equal to `jmpGE <label>`
 * `jmpNLE <label>` - equal to `jmpG <label>`
 * `jmpNE <label>` - jumps to the label if the `LF` flag or the `GF` flag is set
 * `jmpNGE <label>` - equal to `jmpL <label>`
 * `jmpG <label>` - equal to `jmpLE <label>`
 
 There is also two jump instructions for each flag (where `<flag>` is the _Name_ of the flag without the `F`)
 
  * `jmp<flag> <label>` - jumps to the label if the flag is set
  * `jmpN<flag> <label>` - jumps to the label if the flag is not set
 
# Flags

There are a number of flags

 * `CF` - The Carry flag: result of unsigned op. is too large or below zero. 1 = carry/borrow
 * `OF` - The Overflow flag: result of signed op. is too large or small. 1 = overflow/underflow
 * `ZF` - The Zero flag: result of operation is zero. 1 = zero
 * `SF` - The Sign flag: sign of result. Reasonable for signed integer only. 1 = neg. / 0 = pos.
 * `LF` - the Lower flag: set by `cmp op1, op2` and `op1 < op2` was true
 * `EF` - the Equals flag: set by `cmp op1, op2` and `op1 == op2` was true
 * `GF` - the Greater flag: set by `cmp op1, op2` and `op1 > op2` was true
 
 | instruction | `CF` | `OF` | `ZF` | `SF` |
 | --- | --- | --- | --- | --- |
 | add | ± | ± | ± | ± |
 | sub | ± | ± | ± | ± |
 | mul | ? | ± | ± | ± |
 | umul | ? | ± | ± | 1 |
 | div | ? | ± | ± | ± |
 | udiv | ? | ± | ± | 1 |
 
**More to come**
