/**
 * Parse TLVs formatted as specified in Section 14 of the HomeKit Accessory Protocol specification
 */

use std::collections::HashMap;

pub fn parse<'a>(bytes : &'a Vec<u8>) -> HashMap<i32, &'a [u8]> {
    let mut map = HashMap::new();
    _parse(&bytes, &mut map);
    map
}

fn _parse<'a>(bytes : &'a [u8], map : &mut HashMap<i32, &'a [u8]>) {
    let last_byte : usize = (2 + (bytes[1] as usize)).into();
    map.insert(bytes[0].into(), &bytes[2..last_byte]);
    if bytes.len() > last_byte {
        _parse(&bytes[last_byte..], map);
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
    #[ignore]
    fn multipart_value() {
        let bytes = [vec![1, 255], vec![1, 2, 3, 4, 5].repeat(51), vec![1, 5], vec![1,2,3,4,5]].concat();
        let result = parse(&bytes);

        assert_eq!(result.len(), 1);
        let expected : Vec<u8> = vec![1, 2, 3, 4, 5].repeat(52);
        assert_eq!(result.get(&1).unwrap(), &expected);
    }
}
