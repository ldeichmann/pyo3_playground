use log::{info, error};
use pyo3::create_exception;
use pyo3::prelude::*;


create_exception!(string_sum, MyError, pyo3::exceptions::PyException);

#[derive(Debug)]
enum MyErrorEnum {
    Bwoken(String),
    Bwoken2(String),
}


impl From<MyErrorEnum> for PyErr {
    fn from(error: MyErrorEnum) -> Self {
        MyError::new_err(format!("{:?}", error))
    }
}


#[derive(Debug)]
#[pyclass]
struct MyPersistentData {
    #[pyo3(get, set)]
    counter: i32
}

#[pymethods]
impl MyPersistentData {
    #[new]
    fn new(value: i32) -> Self {
        Self {
            counter: value
        }
    }

    // the self argument should be written $self
    #[pyo3(text_signature = "($self, e)")]
    fn add_to_counter<'p>(&self, py: Python<'p>, e: i32) -> PyResult<&'p PyAny> {
        let counter = self.counter;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            Ok((e + counter).to_string())
        })
    }
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: i64, b: i32) -> PyResult<String> {
    Ok((a - b as i64).to_string())
}


/// Formats the sum of two numbers as string but async
#[pyfunction]
fn sum_as_string_but_slow(py: Python, a: i64, b: i32) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok((a - b as i64).to_string())
    })
}


fn my_function() -> Result<i32, MyErrorEnum> {
    Ok(54)
}


#[pyfunction]
fn my_function2() -> Result<(), MyErrorEnum> {
    match my_function() {
        Ok(value) => info!("my_function calculation result {}", value),
        Err(error_enum) => error!("Error occurred: {:?}", error_enum)
    }
    // Ok(())
    Err(MyErrorEnum::Bwoken("It's bwoken!".to_string()))
}

#[pyfunction]
fn my_function3(data: &MyPersistentData) -> Result<(), MyErrorEnum> {
    info!("Received a {:?}", data);
    Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn string_sum(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(my_function2, m)?)?;
    m.add_function(wrap_pyfunction!(my_function3, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string_but_slow, m)?)?;

    m.add("MyError", py.get_type::<MyError>())?;

    m.add_class::<MyPersistentData>()?;
    Ok(())
}