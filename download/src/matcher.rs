pub trait Matcher {
    fn matches(&self, plenarprotokoll: &dip::PlenarprotokollText) -> bool;
}

/// An input parameter that should be interpreted as an Id of a Plenarprotokoll.
pub struct Id(pub String);

impl Matcher for Id {
    fn matches(&self, plenarprotokoll: &dip::PlenarprotokollText) -> bool {
        plenarprotokoll.id == self.0
    }
}
