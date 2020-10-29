use log::debug;

pub trait Loggable {
    fn to_log(&self) -> String;
}

#[inline(always)]
pub fn logged<P>(value: P) -> P
where
    P: Loggable,
{
    debug!(target: "library", "{}", value.to_log());
    value
}
