use std::collections::HashMap;

pub fn parse<'a>(bytes : &'a Vec<u8>) -> HashMap<i32, &'a [u8]> {
    let mut map = HashMap::new();
    _parse(&bytes, &mut map);
    map
}

// TODO - handle multi-section values & repeating keys
fn _parse<'a>(bytes : &'a [u8], map : &mut HashMap<i32, &'a [u8]>) {
    let last_byte : usize = (2 + bytes[1]).into();
    map.insert(bytes[0].into(), &bytes[2..last_byte]);
    if bytes.len() > last_byte {
        _parse(&bytes[last_byte..], map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse() {
        let bytes = &vec![1, 2, 3, 4, 5, 1, 1];
        let result = parse(bytes);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&1).unwrap(), &[3, 4]);
        assert_eq!(result.get(&5).unwrap(), &[1]);
    }
}
