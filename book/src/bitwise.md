# Bitwise


|Sr.No|Operator|Description|Example|
|---|---|---|---|
|1|& (Bitwise AND)|It performs a Boolean AND operation on each bit of its integer arguments.|(A & B) is 2|
|2|\| (BitWise OR)|It performs a Boolean OR operation on each bit of its integer arguments.|(A | B) is 3|
|3|^ (Bitwise XOR)|It performs a Boolean exclusive OR operation on each bit of its integer arguments. Exclusive OR means that either operand one is true or operand two is true, but not both.|(A ^ B) is 1|
|4|! (Bitwise Not)|It is a unary operator and operates by reversing all the bits in the operand.|(!B) is -4|
|5|<< (Left Shift)|It moves all the bits in its first operand to the left by the number of places specified in the second operand. New bits are filled with zeros. Shifting a value left by one position is equivalent to multiplying it by 2, shifting two positions is equivalent to multiplying by 4, and so on.|(A << 1) is 4|
|6|>> (Right Shift)|Binary Right Shift Operator. The left operand’s value is moved right by the number of bits specified by the right operand.|(A >> 1) is 1|
|7|>>> (Right shift with Zero)|This operator is just like the >> operator, except that the bits shifted to the left are always zero.|(A >>> 1) is 1|


```rust
fn main() {
     let a:i32 = 2;     // Bit presentation 10
   let b:i32 = 3;     // Bit presentation 11

   let mut result:i32;

   result = a & b;
   println!("(a & b) => {} ",result);

   result = a | b;
   println!("(a | b) => {} ",result) ;

   result = a ^ b;
   println!("(a ^ b) => {} ",result);

   result = !b;
   println!("(!b) => {} ",result);

   result = a << b;
   println!("(a << b) => {}",result);

   result = a >> b;
   println!("(a >> b) => {}",result);
}
```
