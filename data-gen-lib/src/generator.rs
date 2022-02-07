use crate::physical::physical_types::PhysicalDataType;
use crate::schema::Schema;
use rand::distributions::Distribution;
use rand::Rng;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::marker::PhantomData;

/// A [DataGenerator] is a [Distribution] that can
/// be used to generate realistic looking values
/// of type [T]. Manually creating a generator
/// from a [Schema] is unsafe because it will
/// cause a runtime panic if the schema does
/// not match the runtime type.
///
/// # Examples
///
/// ```
/// use data_gen_lib::generator::DataGenerator;
/// use data_gen_lib::schema::Schema;
/// use data_gen_lib::data_type::DataType;
/// use serde_json::Value;
/// use rand::{thread_rng, Rng};
///
/// let mut schema = Schema::default();
/// schema.with_field("my_field", DataType::Boolean);
///
/// // Creating a generator for Value is always safe
/// let gen = unsafe { DataGenerator::<Value>::new(&schema) };
///
/// println!("{}", thread_rng().sample(&gen))
/// ```
pub struct DataGenerator<'a, T: DeserializeOwned> {
    fields: HashMap<&'a str, PhysicalDataType<'a>>,
    _phantom: PhantomData<T>,
}

impl<'a, T> DataGenerator<'a, T>
where
    T: DeserializeOwned,
{
    /// Creates a new generator for the given type [T].
    ///
    /// # Safety
    ///
    /// The schema must match the type T, otherwise
    /// data generation will `panic!` at runtime.
    pub unsafe fn new(schema: &'a Schema<'a>) -> Self {
        let mut generator = DataGenerator {
            fields: HashMap::new(),
            _phantom: PhantomData::default(),
        };

        for (field, data_type) in schema.iter() {
            generator.fields.insert(field, data_type.into());
        }

        generator
    }
}

#[cfg(feature = "json")]
impl<'a> DataGenerator<'a, Value> {
    pub fn json(schema: &'a Schema<'a>) -> DataGenerator<'a, Value> {
        // verified: generating JSON is always safe
        unsafe { DataGenerator::new(schema) }
    }
}

impl<'a, T> Distribution<T> for DataGenerator<'a, T>
where
    T: DeserializeOwned,
{
    fn sample<'b, R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        let fields = self
            .fields
            .iter()
            .map(|(name, dt)| (name.to_string(), rng.sample(dt)))
            .collect();

        let object = Value::Object(fields);
        let mut buffer = Vec::<u8>::new();
        serde_json::to_writer(&mut buffer, &object)
            .expect("data-gen should never create invalid JSON values. this is a bug.");
        serde_json::from_reader(&buffer[..])
            .expect("data-gen should never fail to deserialize into the final type. this means the safety invariant of `Schema#to_generator` has not been upheld.")
    }
}
