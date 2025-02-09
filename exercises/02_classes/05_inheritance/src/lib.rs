// TODO: Define a base class named `Person`, with `first_name` and `last_name` attributes, set
//  by the constructor. It should be possible to access the `first_name` and `last_name` attributes
//  of a `Person`.
//  `Person` should also have a method named `full_name` that returns the full name of the person.
//  Then define a subclass named `Employee` that inherits from `Person` and adds an
//  unsigned integer `id` attribute and a constructor that sets the `id` attribute.
//  It should be possible to access the `first_name`, `last_name` and `id`
//  attributes of an `Employee`.
use pyo3::prelude::*;


#[pyclass(subclass)]
struct Person {
    #[pyo3(get)]
    first_name: String,
    #[pyo3(get)]
    last_name: String,
}

#[pyclass(extends=Person)]
struct Employee {
    #[pyo3(get)]
    id: u32,
}


#[pymethods]
impl Person {
    #[new]
    fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[pymethods]
impl Employee {
    #[new]
    fn new(
        first_name: String,
        last_name: String,
        id: u32,
    ) -> PyClassInitializer<Self> {
        let sup = Person::new(first_name, last_name);
        let child = Self { id };
        PyClassInitializer::from(sup).add_subclass(child)
    }
}


#[pymodule]
fn inheritance(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Person>()?;
    m.add_class::<Employee>()?;
    Ok(())
}
