use auxtools::{hook, Value};

#[hook("/proc/innit_mole")]
fn innit_mole() {
    Ok(Value::null())
}

#[hook("/proc/mole_say_hello")]
fn mole_say_hello() {
    Ok(Value::from_string("The mole say hello"))
}
