/// Trait that allows callers to add missing info if necessary
pub trait SupplyMissing<T> {
    fn supply_missing(self, info : T) -> Self;
}

impl<T, E, Info> SupplyMissing<Info> for Result<T, E>
    where E : SupplyMissing<Info>
{
    fn supply_missing(self, info: Info) -> Self {
        match self {
            Ok(val) => Ok(val),
            Err(err) => Err(err.supply_missing(info))
        }
    }
}
