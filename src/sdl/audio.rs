
pub struct RingBuffer {
    pub data: Box<[u8]>,
    pub write_cursor: usize,
    pub play_cursor: usize,
}

pub struct OutputState {
    pub samples_per_second: usize,
    pub running_sample_index: usize,
    pub bytes_per_sample: usize,
    pub secondary_buffer_size: usize,
    pub latency_sample_count: usize,
}

// TODO(CJS): Find a much more "rusty" way to do this without having to dip into
//            unsafe buffers...
pub fn audio_callback(
    ring_buffer: &mut RingBuffer,
    data: &[u8],
    length: usize
) {
    let ring_buffer_size = ring_buffer.data.len();

    if ring_buffer.play_cursor + length > ring_buffer_size {
        // NOTE(CJS): this handles the wrap-around case...
        let len_1 = ring_buffer_size - ring_buffer.play_cursor;
        let len_2 = length - len_1;
        unsafe {
            // copy to end of buffer
            let ring_ptr = ring_buffer.data.as_mut_ptr();
            let src_ptr = data.as_ptr();
            let dst_ptr = ring_ptr.offset(ring_buffer.play_cursor as isize);
            let len = len_1;
            ::std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);

            // copy the rest from the beginning of the buffer
            let src_ptr = data.as_ptr();
            let dst_ptr = ring_buffer.data.as_mut_ptr();
            let len = len_2;
            ::std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        }
    } else {
        let len = data.len();
        unsafe {
            let ring_ptr = ring_buffer.data.as_mut_ptr();
            let src_ptr = data.as_ptr();
            let dst_ptr = ring_ptr.offset(ring_buffer.play_cursor as isize);
            ::std::ptr::copy_nonoverlapping (src_ptr, dst_ptr, len);
        }
    }
}

fn gen_wave(bytes_to_write: usize) -> Vec<i16> {
    let tone_volume = 100i16;
    let period = 48000 / 256;
    let sample_count = bytes_to_write;
    let mut samples = Vec::new();
    for t in 0..sample_count {
        samples.push(
            if ( t / period ) % 2 == 0 {
                tone_volume
            } else {
                -tone_volume
            }
        );
    }
    samples
}
