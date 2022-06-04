use crate::{ParamType, WORD_SIZE};

pub fn max_by_encoding_width(params: &[ParamType]) -> Option<usize> {
    params.iter().map(encoding_width).max()
}

fn encoding_width(param: &ParamType) -> usize {
    const fn count_words(bytes: usize) -> usize {
        let q = bytes / WORD_SIZE;
        let r = bytes % WORD_SIZE;
        match r == 0 {
            true => q,
            false => q + 1,
        }
    }

    match param {
        ParamType::Unit => 0,
        ParamType::U8
        | ParamType::U16
        | ParamType::U32
        | ParamType::U64
        | ParamType::Bool
        | ParamType::Byte => 1,
        ParamType::B256 => 4,
        ParamType::Array(param, count) => encoding_width(&param) * count,
        ParamType::String(len) => count_words(*len),
        ParamType::Struct(params) => params.iter().map(encoding_width).sum(),
        ParamType::Enum(variants) => {
            const DISCRIMINANT_WORD_SIZE: usize = 1;

            // Sway ATM doesn't allow empty Enums hence .unwrap()
            let widest_width = max_by_encoding_width(variants).unwrap();

            widest_width + DISCRIMINANT_WORD_SIZE
        }
        ParamType::Tuple(params) => params.iter().map(encoding_width).sum(),
    }
}
