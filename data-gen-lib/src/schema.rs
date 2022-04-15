use crate::data_type::DataType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A [Schema] defines a type and how to generate
/// the values for each field based on the given
/// [DataType].
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Schema<'a> {
    #[serde(flatten, borrow)]
    fields: HashMap<&'a str, DataType<'a>>,
}

impl<'a> Schema<'a> {
    /// An iterator visiting all field-datatype pairs in arbitrary order.
    /// The iterator element type is `(&'a str, &'a DataType<'a>)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_gen_lib::schema::Schema;
    /// use data_gen_lib::data_type::DataType;
    ///
    /// let mut schema = Schema::default();
    /// schema
    ///     .with_field("bool_field", DataType::Boolean)
    ///     .with_field("int_field", DataType::Range { from: 1, to: 3 });
    ///
    /// for (field, data_type) in schema.iter() {
    ///     println!("field: {} data_type: {}", field, data_type);
    /// }
    /// ```
    pub fn iter(&'a self) -> impl Iterator<Item = (&'a str, &'a DataType<'a>)> {
        let mut v = Vec::from_iter(self.fields.iter());
        v.sort_by(|(left, _), (right, _)| left.cmp(right));
        v.into_iter().map(|(name, tpe)| (*name, tpe))
    }

    /// Adds a new field to the [Schema]. If a field
    /// with the given name already exists it will
    /// be overridden.
    ///
    /// # Examples
    ///
    ///```
    /// use data_gen_lib::schema::Schema;
    /// use data_gen_lib::data_type::DataType;
    ///
    /// let mut schema = Schema::default();
    /// schema.with_field("bool_field", DataType::Boolean);
    ///```
    pub fn with_field(&mut self, name: &'a str, dt: DataType<'a>) -> &mut Self {
        self.fields.insert(name, dt);
        self
    }
}
