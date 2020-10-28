use bencher::Bencher;

use integer_encoding::*;

fn encode_v(b: &mut Bencher) {
    let my_u64 = 94949291991190 as u64;
    let my_i64 = -12219304371120 as i64;

    let mut dst = [0 as u8; 10];

    b.iter(|| {
        // 8x each.
        my_u64.encode_var(&mut dst);
        my_u64.encode_var(&mut dst);
        my_u64.encode_var(&mut dst);
        my_u64.encode_var(&mut dst);
        my_u64.encode_var(&mut dst);
        my_u64.encode_var(&mut dst);
        my_u64.encode_var(&mut dst);

        my_i64.encode_var(&mut dst);
        my_i64.encode_var(&mut dst);
        my_i64.encode_var(&mut dst);
        my_i64.encode_var(&mut dst);
        my_i64.encode_var(&mut dst);
        my_i64.encode_var(&mut dst);
        my_i64.encode_var(&mut dst);
        my_i64.encode_var(&mut dst);
    });
}

fn decode_v(b: &mut Bencher) {
    let my_u64 = 94949291991190 as u64;
    let my_i64 = -12219304371120 as i64;

    let u64_src = my_u64.encode_var_vec();
    let i64_src = my_i64.encode_var_vec();

    b.iter(|| {
        // 8x each.
        u64::decode_var(&u64_src).unwrap();
        u64::decode_var(&u64_src).unwrap();
        u64::decode_var(&u64_src).unwrap();
        u64::decode_var(&u64_src).unwrap();
        u64::decode_var(&u64_src).unwrap();
        u64::decode_var(&u64_src).unwrap();
        u64::decode_var(&u64_src).unwrap();

        i64::decode_var(&i64_src).unwrap();
        i64::decode_var(&i64_src).unwrap();
        i64::decode_var(&i64_src).unwrap();
        i64::decode_var(&i64_src).unwrap();
        i64::decode_var(&i64_src).unwrap();
        i64::decode_var(&i64_src).unwrap();
        i64::decode_var(&i64_src).unwrap();
        i64::decode_var(&i64_src).unwrap();
    });
}

bencher::benchmark_group!(varint_benches, encode_v, decode_v);

fn encode_f(b: &mut Bencher) {
    let my_u64 = 94949291991190 as u64;
    let my_i64 = -12219304371120 as i64;

    let mut dst = [0 as u8; 8];

    b.iter(|| {
        // 8x each.
        my_u64.encode_fixed(&mut dst);
        my_u64.encode_fixed(&mut dst);
        my_u64.encode_fixed(&mut dst);
        my_u64.encode_fixed(&mut dst);
        my_u64.encode_fixed(&mut dst);
        my_u64.encode_fixed(&mut dst);
        my_u64.encode_fixed(&mut dst);

        my_i64.encode_fixed(&mut dst);
        my_i64.encode_fixed(&mut dst);
        my_i64.encode_fixed(&mut dst);
        my_i64.encode_fixed(&mut dst);
        my_i64.encode_fixed(&mut dst);
        my_i64.encode_fixed(&mut dst);
        my_i64.encode_fixed(&mut dst);
        my_i64.encode_fixed(&mut dst);
    });
}

fn decode_f(b: &mut Bencher) {
    let my_u64 = 94949291991190 as u64;
    let my_i64 = -12219304371120 as i64;

    let u64_src = my_u64.encode_fixed_vec();
    let i64_src = my_i64.encode_fixed_vec();

    b.iter(|| {
        // 8x each.
        u64::decode_fixed(&u64_src);
        u64::decode_fixed(&u64_src);
        u64::decode_fixed(&u64_src);
        u64::decode_fixed(&u64_src);
        u64::decode_fixed(&u64_src);
        u64::decode_fixed(&u64_src);
        u64::decode_fixed(&u64_src);

        i64::decode_fixed(&i64_src);
        i64::decode_fixed(&i64_src);
        i64::decode_fixed(&i64_src);
        i64::decode_fixed(&i64_src);
        i64::decode_fixed(&i64_src);
        i64::decode_fixed(&i64_src);
        i64::decode_fixed(&i64_src);
        i64::decode_fixed(&i64_src);
    });
}

bencher::benchmark_group!(fixedint_benches, encode_f, decode_f);

bencher::benchmark_main!(varint_benches, fixedint_benches);
