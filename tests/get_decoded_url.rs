use std::boxed::Box;

#[test]
fn get_decoded_target() {
    let target = b"/part1/part2+space/part3///";
    let decoded_strings_buffer = unsafe { Box::<[u8]>::new_uninit_slice(0x4000).assume_init() };
    let target_parts_vec_buffer = unsafe { Box::<[usize]>::new_uninit_slice(0x400).assume_init() };

    explode_target(target);
}

fn explode_target(target: &[u8]) {
    let mut part_num = 0;
    let mut i = 0;
    let mut p_start = 0;
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
        println!("part {part_num}: {:?}", str::from_utf8(c_part));
        part_num += 1;
    }
}
