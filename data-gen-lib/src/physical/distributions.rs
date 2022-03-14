use std::cell::RefCell;

use rand::distributions::Distribution;
use rand::{Rng, RngCore};
use serde_json::Value;

/// A trait object safe wrapper for a [Distribution].
pub trait DynDistribution {
    fn sample_(&self, rng: &mut dyn RngCore) -> Value;
}

/// A blanket implementation for all [Distribution]s
/// that return [Value] elements.
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
pub struct Static(Value);

impl Static {
    pub fn new<T>(value: T) -> Self
    where
        T: AsRef<str>,
    {
        Static(Value::String(value.as_ref().to_string()))
    }
}

impl Distribution<Value> for Static {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Value {
        self.0.clone()
    }
}

/// A distribution that proxies to a
/// function and does not rely on a [Rng]
/// instance to compute its values.
pub struct Supplier(Box<dyn Fn() -> Value>);

impl Supplier {
    pub fn new<T>(supplier: T) -> Self
    where
        T: Fn() -> Value,
        T: 'static,
    {
        Supplier(Box::new(supplier))
    }
}

impl Distribution<Value> for Supplier {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Value {
        self.0()
    }
}

/// A distribution that pulls elements
/// from an underlying [Iterator]. When
/// the iterator is exausted the distribution
/// will return [Value::Null].
pub struct Iter(RefCell<Box<dyn Iterator<Item = Value>>>);

impl Iter {
    pub fn new<T: 'static>(iterator: T) -> Iter
    where
        T: IntoIterator<Item = Value>,
    {
        let boxed = Box::new(iterator.into_iter());
        Iter(RefCell::new(boxed))
    }
}

impl Distribution<Value> for Iter {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Value {
        let mut iter = self.0.borrow_mut();
        match iter.next() {
            Some(v) => v,
            None => Value::Null,
        }
    }
}

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};
    use serde_json::{json, to_value, Value};
    use std::vec;

    use super::{Iter, Static, Supplier};

    #[test]
    fn test_static_distribution() {
        let distribution = Static(json!("hello"));
        let results: Vec<_> = thread_rng().sample_iter(distribution).take(2).collect();

        assert_eq!(vec!["hello", "hello"], results)
    }

    #[test]
    fn test_supplier_distribution() {
        let distribution = Supplier(Box::new(|| Value::String("hello".to_string())));
        let results: Vec<_> = thread_rng().sample_iter(distribution).take(2).collect();

        assert_eq!(vec!["hello", "hello"], results)
    }

    #[test]
    fn test_iterable_distribution() {
        let mut expected: Vec<_> = vec![1, 2, 3]
            .into_iter()
            .map(|value: i64| to_value(value).unwrap())
            .collect();

        let distribution = Iter::new(expected.clone());

        let results: Vec<_> = thread_rng().sample_iter(distribution).take(4).collect();

        expected.push(Value::Null);

        assert_eq!(expected, results)
    }
}
