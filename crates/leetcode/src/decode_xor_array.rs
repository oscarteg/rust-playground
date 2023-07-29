// encoded[i] = arr[i] XOR arr[i + 1]
pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    // Create vector with size of encoded + 1
    let mut result = vec![0; encoded.len() + 1];

    // set first as the first value
    result[0] = first;

    // loop from 1 to end of result (aka lenght of encoded + 1)
    for i in 1..encoded.len() + 1 {
        result[i] = result[i - 1] ^ encoded[i - 1];
    }

    result
}

#[test]
fn simple() {
    let encoded = vec![1, 2, 3];
    let first = 1;

    assert_eq!(decode(encoded, first), vec![1, 0, 2, 1]);
}
