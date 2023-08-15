use std::convert::From;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PAGVideoRange {
    pub start_time: i64,
    pub end_time: i64,
    pub play_duration: i64,
    pub reversed: bool,
}

impl From<&ffi::pag::PAGVideoRange> for PAGVideoRange {
    fn from(value: &ffi::pag::PAGVideoRange) -> Self {
        Self {
            start_time: value.startTime(),
            end_time: value.endTime(),
            play_duration: value.endTime(),
            reversed: value.reversed(),
        }
    }
}
