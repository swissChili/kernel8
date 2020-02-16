use crate::io::serial;

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    serial::writeln("Running Tests");
    for test in tests {
        test();
        serial::writeln("... Ok");
    }
}
