pub struct PythonBindingMeta;
impl PythonBindingMeta {
    pub fn get_module_name() -> &'static str { "sigmaos_xapp" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_python_binding_meta() {
        assert_eq!(PythonBindingMeta::get_module_name(), "sigmaos_xapp");
    }
}
