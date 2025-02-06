use pyo3::prelude::*;
use pyo3::types::PyInt;
use pyo3::exceptions::PyTypeError;


// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
//  It must raise a `TypeError` if `n` is not an integer or if it is less than 0.
#[pyfunction]
fn fibonacci(n: Bound<'_, PyInt>) -> PyResult<Vec<u64>> {
    let n: usize = n
        .extract()
        .map_err(|_| PyTypeError::new_err("n must be an integer"))?;

    let sequence = match n {
        0 => Vec::new(),
        1 => vec![0],
        2 => vec![0, 1],
        _ => {
            let mut seq = vec![0, 1];
            for i in 2..n {
                let next = seq[i - 1] + seq[i - 2];
                seq.push(next);
            }
            seq
        }
    };
    Ok(sequence)
}


#[pymodule]
fn exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
