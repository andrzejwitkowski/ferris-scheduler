pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::with_closure;
    
    struct TestProcessor;
    impl TestProcessor {
        fn process(&self, a: i32, b: i32, f: impl Fn(i32) -> i32) -> i32 {
            f(a + b)
        }
    }

    fn calculate(a: i32, b: i32, op: impl Fn(i32, i32) -> i32) -> i32 {
        op(a, b)
    }

    #[test]
    fn test_with_closure_method() {
        let processor = TestProcessor;
        
        let result = with_closure! [
            processor => process(10, 20) with {
                x -> x * 2
            }
        ];
        
        assert_eq!(result, 60);
    }
    
    #[test]
    fn test_with_closure_function() {
        let result = with_closure![
            calculate(15, 3) with {
                a, b -> a * b
            }
        ];
        
        assert_eq!(result, 45);
    }
    
    
}
