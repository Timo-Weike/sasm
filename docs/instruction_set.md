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
 
**More to come**
