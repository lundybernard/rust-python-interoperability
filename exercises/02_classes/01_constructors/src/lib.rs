use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;


// TODO: Add a `__new__` constructor to the `ShoppingOrder` class that takes the following arguments:
//  - `name` (non-empty string)
//  - `price` (non-zero integer)
//  - `quantity` (non-zero integer)
//  The constructor should raise a `ValueError` if any of the arguments are invalid.

#[pyclass]
struct ShoppingOrder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    price: i64,
    #[pyo3(get, set)]
    quantity: i64,
}

#[pymethods]
impl ShoppingOrder {
    #[new]
    fn new(name: String, price: i64, quantity: i64) -> PyResult<Self> {
        if name.is_empty() {
            return Err(PyValueError::new_err("Name cannot be empty"));
        }
        if price <= 0 {
            return Err(PyValueError::new_err("Price must be positive"));
        }
        if quantity <= 0 {
            return Err(PyValueError::new_err("Quantity must be positive"));
        }

        Ok(Self { name, price, quantity })
    }
}

#[pymodule]
fn constructors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    Ok(())
}
