use rhai::{Engine, EvalAltResult};

pub fn new_engine() -> Engine {
    Engine::new()
}

pub fn register_fns(engine: &mut Engine) {
}

fn contains(content: &str, sub: &str) -> bool {
    content.contains(sub)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_function_registration() {
        let mut engine = new_engine();
        register_fns(&mut engine);
        
        // 测试注册的函数是否能正常工作
        let result: bool = engine.eval(r#"contains("hello world", "hello")"#).unwrap();
        assert!(result);
        
        let result: bool = engine.eval(r#"contains("hello world", "foo")"#).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_contains_function() {
        assert!(contains("hello world", "hello"));
        assert!(!contains("hello world", "foo"));
    }
}
