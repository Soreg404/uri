use std::boxed::Box;

#[test]
fn get_decoded_target() {
    let target = b"/part1/part2+space/part3///";
    let mut decoded_strings_buffer = unsafe { Box::<[u8]>::new_uninit_slice(0x4000).assume_init() };
    let mut target_parts_vec_buffer = unsafe { Box::<[usize]>::new_uninit_slice(0x400).assume_init() };

    let n_parts = explode_target(
        target,
        &mut decoded_strings_buffer,
        &mut target_parts_vec_buffer
    );

    println!("====== RESULT ======");

    println!("num_parts: {n_parts}");
    let mut head = 0;
    for i in 0..n_parts {
        let c_len = target_parts_vec_buffer[i];
        println!("part: {:?}", str::from_utf8(
                &decoded_strings_buffer[head..head + c_len]));
        head += c_len;
    }
}

fn explode_target(
    target: &[u8],
    b_buf: &mut [u8],
    p_buf: &mut [usize]
) -> usize {
    let mut part_num = 0;
    let mut i = 0;
    let mut p_start = 0;
    let mut b_buf_head = 0;
    while i < target.len() {
        while i < target.len() && target[i] == b'/' {
            i += 1;
        }

        p_start = i;
        while i < target.len() && target[i] != b'/' {
            i += 1;
        }
        if p_start == i {
            println!("end");
            break;
        }
        let c_part = &target[p_start..i];
        b_buf[b_buf_head..b_buf_head + c_part.len()].copy_from_slice(c_part);
        b_buf_head += c_part.len();
        p_buf[part_num] = c_part.len();
        println!("part {part_num}: {:?}", str::from_utf8(c_part));
        part_num += 1;
    }

    part_num
}
