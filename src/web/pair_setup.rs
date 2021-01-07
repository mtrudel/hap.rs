use std::collections::HashMap;

#[allow(non_upper_case_globals)]
pub const kTLVType_State: i32 = 6;

pub fn m1(params : &HashMap<i32, Vec<u8>>) -> HashMap<i32, Vec<u8>> {
    println!("{:?}", params);
    HashMap::new()
}
