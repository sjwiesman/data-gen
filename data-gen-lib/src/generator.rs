use crate::physical::physical_types::PhysicalDataType;
use crate::schema::Schema;
use rand::distributions::Distribution;
use rand::Rng;
use serde_json::Value;
use std::collections::HashMap;

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
/// let gen = DataGenerator::new(&schema);
///
/// println!("{}", thread_rng().sample(&gen))
/// ```
pub struct DataGenerator<'a> {
    fields: HashMap<&'a str, PhysicalDataType<'a>>,
}

impl<'a> DataGenerator<'a> {
    /// Creates a new generator.
    pub fn new(schema: &'a Schema<'a>) -> Self {
        let mut generator = DataGenerator {
            fields: HashMap::new(),
        };

        for (field, data_type) in schema.iter() {
            generator.fields.insert(field, data_type.into());
        }

        generator
    }
}

impl<'a> Distribution<Value> for DataGenerator<'a> {
    fn sample<'b, R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        let fields = self
            .fields
            .iter()
            .map(|(name, dt)| (name.to_string(), rng.sample(dt)))
            .collect();

        Value::Object(fields)
    }
}
