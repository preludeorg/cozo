use miette::Result;

use crate::{DataValue, SourceSpan};

pub trait Encoder: Send {
    fn encode_key_for_store(&self, tuple: &[DataValue], span: SourceSpan) -> Result<Vec<u8>>;
    fn encode_val_for_store(&self, tuple: &[DataValue], _span: SourceSpan) -> Result<Vec<u8>>;
}
