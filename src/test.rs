#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    use crate::*;

    println!("Running Tests");
    for test in tests {
        test();
        println!("... Ok");
    }
}
