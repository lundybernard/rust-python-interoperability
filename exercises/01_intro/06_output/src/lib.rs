use pyo3::prelude::*;

// I'm a bit surprised this works.  guessing PyResult performs the conversion
// from Vec<u64> to a Python List object
#[pyfunction]
fn fibonacci(n: usize) -> Vec<u64> {
    match n {
        0 => Vec::new(),
        1 => vec![0],
        2 => vec![0, 1],
        _ => {
            let mut sequence = vec![0, 1];
            for i in 2..n {
                let next = sequence[i - 1] + sequence[i - 2];
                sequence.push(next);
            }
            sequence
        }
    }
}


#[pymodule]
fn output(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
