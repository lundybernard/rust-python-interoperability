use pyo3::prelude::*;

// TODO: Create a new Python class named `ShoppingOrder` with the following attributes:
//  - `price` (positive integer)
//  - `quantity` (positive integer)
//  - `name` (string)
//  Expose a `new_order` function to create a new `ShoppingOrder` instance.
//  It shouldn't be possible to modify the name or the price after the object is created.


// Dev Notes: how to test manually
// maturin develop --manifest-path exercises/02_classes/00_pyclass/Cargo.toml
// pytest exercises/02_classes/00_pyclass/

#[pyclass]
struct ShoppingOrder {
    #[pyo3(get)]
    price: u32,
    #[pyo3(get, set)]
    quantity: u32,
    #[pyo3(get)]
    name: String,
}


#[pyfunction]
fn new_order(name: String, price: u32, quantity: u32, ) -> ShoppingOrder {
    ShoppingOrder{price, quantity, name,}
}


#[pymodule]
fn pyclass(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    m.add_function(wrap_pyfunction!(new_order, m)?)?;
    Ok(())
}


// === Unit Tests === //

// Not really top-quality unit tests, experimenting, and learning.
#[cfg(test)]
mod tests {
    use super::*; // Import your module
    use pyo3::prelude::*;


    #[test]
    fn test___init__() {
        pyo3::prepare_freethreaded_python(); // Initialize Python interpreter

        Python::with_gil(|py| {
            let shopping_order = new_order(
                "test-order".to_string(), 42, 7
            );
            assert_eq!(shopping_order.name, "test-order");
            assert_eq!(shopping_order.price, 42);
            assert_eq!(shopping_order.quantity, 7);

            // Test setter (using setattr)
            // the setter is only available through the python object wrapper
        });
    }
}
