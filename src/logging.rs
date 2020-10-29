use flexi_logger::{DeferredNow, Record};
use log::debug;

pub trait Loggable {
    fn to_log(&self) -> String;
}

pub fn test_format(
    w: &mut dyn std::io::Write,
    _: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(w, "{}", &record.args())
}

#[inline(always)]
pub fn logged<P>(value: P) -> P
where
    P: Loggable,
{
    debug!(target: "library", "{}", value.to_log());
    value
}
