#![no_main]

use libfuzzer_sys::fuzz_target;
use integer_encoding::VarInt;

fuzz_target!(|data: &[u8]| {

     // unfortunately decode_var can't be round-trip tested, so just test for panic.
     let _ = u64::decode_var(data);
     let _ = i64::decode_var(data);

    fn test_round_trip<T: VarInt>(data: &[u8]) {
        match T::decode_var_exact(data, false) {
            Some((val, _)) => {
               let v = val.encode_var_vec();
               assert_eq!(&v, &data[0..v.len()]);
            },
            None => {
                // This is fine as long as it doesn't panic
            }
        }
    }


     match u64::decode_var_exact(data, false) {
        Some((val, _)) => {
            let v = val.encode_var_vec();
            if &v != &data[0..v.len()] {
                dbg!(val);
            }
            assert_eq!(&v, &data[0..v.len()]);
        },
        None => {
            // This is fine as long as it doesn't panic
        }
    }
    test_round_trip::<u64>(data);
    test_round_trip::<i64>(data);
    test_round_trip::<u32>(data);
    test_round_trip::<i32>(data);
});
