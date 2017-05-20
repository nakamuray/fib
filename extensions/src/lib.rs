#[macro_use]
extern crate cpython;
extern crate rayon;

use cpython::{PyResult, Python};

py_module_initializer!(fib, initfib, PyInit_fib, |py, m| {
    m.add(py, "fib", py_fn!(py, fib_py(i: i64)))?;
    Ok(())
});

fn fib(i: i64) -> i64 {
    if i < 0 {
        panic!("undefined");
    }
    match i {
        0 | 1 => i,
        _ => {
            let (j, k) = rayon::join(|| fib(i - 1), || fib(i - 2));
            j + k
        }
    }
}

fn fib_py(_: Python, i: i64) -> PyResult<i64> {
    Ok(fib(i))
}

#[cfg(test)]
mod tests {
    use fib;
    #[test]
    fn it_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
    }
}
