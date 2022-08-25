use crate::bytecode::chunk::Value;

macro_rules! impl_value {
    ($($T:ty ; -> ($c:tt)),+) => {$(
        impl Value<$c> for $T {
            fn to_bits(self) -> [u8; $c] {
                self.to_le_bytes()
            }

            fn from_bits(bits: [u8; $c]) -> Self {
                <$T>::from_le_bytes(bits)
            }
        }
    )+};
}


impl_value!(
    u8; -> (1),
    i8; -> (1),

    u16; -> (2),
    i16; -> (2),

    u32; -> (4),
    i32; -> (4),
    f32; -> (4),

    u64; -> (8),
    i64; -> (8),
    f64; -> (8),

    u128; -> (16),
    i128; -> (16)
);
