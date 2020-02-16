use crate::*;

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    println!("Running Tests");
    for test in tests {
        test();
        println!("... Ok");
    }
}
