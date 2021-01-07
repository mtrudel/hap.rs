/**
 * Parse TLVs formatted as specified in Section 14 of the HomeKit Accessory Protocol specification
 */

use std::collections::HashMap;

pub fn parse(bytes : &Vec<u8>) -> HashMap<i32, Vec<u8>> {
    // Use a list of tuples so we can handle multi-occurrences of specific keys,
    // as comes up in the list-pairings endpoint
    let mut entries : Vec<(i32, Vec<u8>)> = Vec::new();
    _parse(&bytes, &mut entries);
    entries.into_iter().collect()
}

fn _parse(bytes : &[u8], entries : &mut Vec<(i32, Vec<u8>)>) {
    let last_byte : usize = (2 + (bytes[1] as usize)).into();
    let tag : i32 = bytes[0].into();
    let value = &bytes[2..last_byte];

    match entries.last_mut() {
        Some((prev_tag, prev_value)) if prev_tag == &tag =>
            prev_value.extend_from_slice(value),

        _ =>
            entries.push((tag, value.to_vec()))
    }

    if bytes.len() > last_byte {
        _parse(&bytes[last_byte..], entries);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_values() {
        let bytes = vec![1, 1, 1, 2, 1, 2];
        let result = parse(&bytes);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&1).unwrap(), &[1]);
        assert_eq!(result.get(&2).unwrap(), &[2]);
    }

    #[test]
    fn empty_value() {
        let bytes = vec![1, 0];
        let result = parse(&bytes);

        assert_eq!(result.len(), 1);
        let empty : Vec<u8> = vec![];
        assert_eq!(result.get(&1).unwrap(), &empty);
    }

    #[test]
    fn large_value() {
        let bytes = [vec![1, 255], vec![1, 2, 3, 4, 5].repeat(51)].concat();
        let result = parse(&bytes);

        assert_eq!(result.len(), 1);
        let expected : Vec<u8> = vec![1, 2, 3, 4, 5].repeat(51);
        assert_eq!(result.get(&1).unwrap(), &expected);
    }

    #[test]
    fn multipart_value() {
        let bytes = [vec![1, 255], vec![1, 2, 3, 4, 5].repeat(51), vec![1, 5], vec![1,2,3,4,5]].concat();
        let result = parse(&bytes);

        assert_eq!(result.len(), 1);
        let expected : Vec<u8> = vec![1, 2, 3, 4, 5].repeat(52);
        assert_eq!(result.get(&1).unwrap(), &expected);
    }
}
