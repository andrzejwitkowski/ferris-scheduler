// Re-export the macro for use in your crate
pub use task_definition_macro::task_definition;

pub mod domain;
pub mod prelude;

#[cfg(test)]
mod tests {
    use trailing_closure_macro::with_block;
    
    struct TestProcessor;
    impl TestProcessor {
        fn process(&self, a: i32, b: i32, f: impl Fn(i32) -> i32) -> i32 {
            f(a + b)
        }
    }

    fn calculate(a: i32, b: i32, op: impl Fn(i32, i32) -> i32) -> i32 {
        op(a, b)
    }

    fn higher_order(a: i32, f: impl Fn(i32, &str) -> String) -> String {
        assert!(a > 0);
        f(42, "test")
    }

    #[test]
    fn test_closure_with_parameters_no_types() {
        
        let result = with_block! {higher_order(1) {
            |num, text| format!("Number: {}, Text: {}", num, text)
        }};
        
        assert_eq!(result, "Number: 42, Text: test");
    }

    #[test]
    fn test_with_closure_function() {
        let result = with_block! {
            calculate(15, 3) {
                |a, b| a * b
            }
        };
        
        assert_eq!(result, 45);
    }

    #[test]
    fn test_with_closure_method() {
        let processor = TestProcessor;
        
        let result = with_block! {
            processor.process(10, 20) {
                |x| x * 2 
            }
        };
        
        assert_eq!(result, 60);
    }    
}
