use rand::distributions::Distribution;
use rand::{Rng, RngCore};
use serde_json::Value;

/// A trait object safe wrapper for a [Distribution].
pub trait DynDistribution {
    fn sample_(&self, rng: &mut dyn RngCore) -> Value;
}

impl<'a> From<&'a str> for Box<dyn DynDistribution> {
    fn from(value: &'a str) -> Self {
        Box::new(Static(Value::String(value.to_string())))
    }
}

impl<T: 'static> From<T> for Box<dyn DynDistribution>
where
    T: Fn() -> Value,
{
    fn from(f: T) -> Self {
        Box::new(Supplier(Box::new(f)))
    }
}

impl<D> DynDistribution for D
where
    D: Distribution<Value>,
{
    fn sample_(&self, rng: &mut dyn RngCore) -> Value {
        <Self as Distribution<Value>>::sample(self, rng)
    }
}

impl Distribution<Value> for dyn DynDistribution + '_ {
    fn sample<R: Rng + ?Sized>(&self, mut rng: &mut R) -> Value {
        self.sample_(&mut rng)
    }
}

/// A [Static] distribution that
/// always returns the same value.
struct Static(Value);

impl Distribution<Value> for Static {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Value {
        self.0.clone()
    }
}

/// A distribution that proxies to a
/// function and does not rely on a [Rng]
/// instance to compute its values.
pub struct Supplier(Box<dyn Fn() -> Value>);

impl Distribution<Value> for Supplier {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Value {
        self.0()
    }
}
