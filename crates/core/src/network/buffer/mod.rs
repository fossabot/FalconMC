mod read;
mod write;

pub use read::{ByteLimitCheck, PacketBufferRead};
pub use write::PacketBufferWrite;

/// Returns the amount of bytes needed to encode this i32 in [var-int](https://wiki.vg/Protocol#VarInt_and_VarLong) format.
pub fn get_var_i32_size(input: i32) -> usize {
    for i in 1..5 {
        if (input & ((-1 as i32) << i * 7)) == 0 {
            return i;
        }
    }
    5
}

/// Returns the amount of bytes needed to encode this i64 in [var-int](https://wiki.vg/Protocol#VarInt_and_VarLong) format.
pub fn get_var_i64_size(input: i64) -> usize {
    for i in 1..10 {
        if (input & ((-1 as i64) << i * 7)) == 0 {
            return i;
        }
    }
    10
}
