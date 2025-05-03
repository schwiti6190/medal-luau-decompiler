use super::list::parse_list;
use nom::{
    number::complete::{le_f32, le_f64, le_u32, le_u8},
    IResult,
};
use nom_leb128::leb128_usize;

#[derive(Debug)]
pub struct Variable {
    pub name_ix : usize,
    pub scope_a : usize,
    pub scope_b : usize,
    pub register : u8,
}

impl Variable {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, name_ix) = leb128_usize(input)?;
        let (input, scope_a) = leb128_usize(input)?;
        let (input, scope_b) = leb128_usize(input)?;
        let (input, register) = le_u8(input)?;
        Ok((input, Self {name_ix, scope_a, scope_b, register }))
    }
}
