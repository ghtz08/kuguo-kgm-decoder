mod kugou;

use std::io::Read;
use kugou::KuGou;

pub fn new<'a>(data: impl Read + 'a) -> Option<impl Decoder<'a>> {
    if let Some(val) = KuGou::try_new(data) {
        Some(val)
    } else {
        None
    }
}

pub trait Decoder<'a>: Sized + Read {
    fn new(origin: impl Read + 'a) -> Self;
    fn try_new(origin: impl Read + 'a) -> Option<Self>;
    fn decodeable_length_interval() -> (u64, u64);
}
