# Instruction Set

## Interger instructions:

 * `add<width> <op>, <op>`
 * `sub<width> <op>, <op>`
 * `mul<width> <op>, <op>`
 * `div<width> <op>, <op>`
 * `inc<width> <op>`
 * `dec<width> <op>`
 
 `ToDo - unsigned and signed mul/div`
 
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
 
 Many assambler languages have a `mov` instruction that is used to move value between registers and the memory.
 That sometimes leaves room for human misinterpretation. So we decided to have two distinct instruction to _load_ a value to a register and _store_ instruction to write a value to the memory.
 
  * `load <reg>, <op>` here `<op>` - can be an addressing of memory or another register. For example `load r1_16, [r2+19]` loads 16 bit from the address `r2+19` into the register `r1_16` and `load r2_8, r3_8` loads the value from the register `r3_8` into the register `r2_8`.
  * `store<width> <addr>, <op>` - here the `width` can be ommited if the second operand is an register. For example `store [r1], r2` writes the value in `r2` to the memory address stored in `r1` and `store32 [r4], [r4+4]` writes 32 begining by `r4+4` to the memory location `r4`.

There is also `copy` instruction that can copy more than 8 byte of data around.

 * `copy <reg>, <addr>, <addr>`
 
The first operand holds the amount of bytes to copy, the second is the base address for the source and the third is the base address for the destination. This instruction might need as much cycels to as bytes to copy to complete.

## Stack operations

 * `pop<width> <reg>`
 * `popFlags`
 * `popAll`
 * `push <reg>`
 * `pushFlags`
 * `pushAll`
 
**More to come**
