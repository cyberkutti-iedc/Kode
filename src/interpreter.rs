use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::ast::{Statement, Expression, BinaryOp, UnaryOp};
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Float(f64),
    Bool(bool),
    Void,
    String(String),
    Array(Vec<Value>),
    Closure {
        params: Vec<String>,
        body: Vec<Statement>,
        env: Environment,
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Void => write!(f, "void"),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            },
            Value::Closure { .. } => write!(f, "<function>"),
        }
    }
}

#[derive(Debug, Clone)]
struct Function {
    file_prefix: String,
    name: String,
    is_main: bool,
    params: Vec<String>,
    body: Vec<Statement>,
}

type Environment = Vec<HashMap<String, Value>>;

pub struct Interpreter {
    variables: Environment,
    functions: HashMap<String, Function>,
    imported_modules: HashMap<String, HashMap<String, Function>>,
    current_function: Option<String>, // Track current function for better error messages
    base_path: PathBuf,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: vec![HashMap::new()],
            functions: HashMap::new(),
            imported_modules: HashMap::new(),
            current_function: None,
            base_path: PathBuf::from("."),
        }
    }

    pub fn set_base_path(&mut self, path: &Path) {
        self.base_path = path.to_path_buf();
    }

    pub fn run(&mut self, program: Vec<Statement>) -> Result<(), String> {
        // First pass: register function definitions and handle imports
        for stmt in &program {
            match stmt {
                Statement::FunctionDef { file_prefix, is_main, name, params, body } => {
                    let func_name = if *is_main {
                        "main".to_string()
                    } else {
                        name.clone()
                    };
                    
                    self.functions.insert(
                        func_name.clone(), 
                        Function { 
                            file_prefix: file_prefix.clone(),
                            name: func_name,
                            is_main: *is_main,
                            params: params.clone(), 
                            body: body.clone() 
                        }
                    );
                }
                Statement::Import(module_name) => {
                    self.import_module(module_name)?;
                }
                _ => {}
            }
        }

        // Check for main function as entry point
        let has_main = self.functions.contains_key("main");
        let has_app = self.functions.contains_key("app");
        
        if !has_main && !has_app {
            return Err("No entry point found. Either 'main' function or 'app' function is required.".to_string());
        }
        
        // Execute main or app function as the entry point
        let entry_point = if has_main { "main" } else { "app" };
        self.call_function(entry_point, vec![])?;
        
        Ok(())
    }

    fn import_module(&mut self, module_name: &str) -> Result<(), String> {
        // Construct the path to the module
        let mut module_path = self.base_path.clone();
        module_path.push(format!("{}.kode", module_name));
        
        let module_path_str = module_path.to_string_lossy().to_string();
        
        // Check if the module exists
        if !module_path.exists() {
            return Err(format!("Module '{}' not found at path '{}'", module_name, module_path_str));
        }
        
        // Read and parse the module
        let source_code = match fs::read_to_string(&module_path) {
            Ok(contents) => contents,
            Err(e) => return Err(format!("Error reading module '{}': {}", module_name, e)),
        };
        
        let mut parser = match Parser::new(&module_path_str, &source_code) {
            Ok(p) => p,
            Err(e) => return Err(format!("Parser error on module '{}': {}", module_name, e)),
        };
        
        let module_ast = match parser.parse_module() {
            Ok(ast) => ast,
            Err(e) => return Err(format!("Parse error on module '{}': {}", module_name, e)),
        };
        
        // Extract function definitions from the module
        let mut module_functions = HashMap::new();
        
        for stmt in module_ast {
            if let Statement::FunctionDef { file_prefix, is_main: _, name, params, body } = stmt {
                module_functions.insert(
                    name.clone(), 
                    Function { 
                        file_prefix, 
                        name: name.clone(),
                        is_main: false, // Imported functions are never main
                        params, 
                        body 
                    }
                );
            }
        }
        
        self.imported_modules.insert(module_name.to_string(), module_functions);
        Ok(())
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        // First, check if it's a local function
        if let Some(function) = self.functions.get(name).cloned() {
            return self.execute_function(function, args);
        }
        
        // Check in imported modules
        for (module_name, module_functions) in &self.imported_modules {
            if let Some(function) = module_functions.get(name).cloned() {
                return self.execute_function(function, args);
            }
        }
        
        Err(format!("Undefined function '{}'", name))
    }
    
    fn execute_function(&mut self, function: Function, args: Vec<Value>) -> Result<Value, String> {
        let previous_function = self.current_function.clone();
        self.current_function = Some(function.name.clone());
        
        if args.len() != function.params.len() {
            return Err(format!(
                "Function '{}' expects {} args, got {}",
                function.name, function.params.len(), args.len()
            ));
        }

        // New function scope
        let mut scope = HashMap::new();
        for (param, arg) in function.params.iter().zip(args) {
            scope.insert(param.clone(), arg);
        }
        self.variables.push(scope);

        let mut return_value = Value::Void;

        // Loop through each statement in the function body
        for (i, stmt) in function.body.iter().enumerate() {
            match self.eval_statement(stmt) {
                Ok(Some(val)) => {
                    return_value = val;
                    break;
                }
                Ok(None) => {}
                Err(e) => {
                    // Add context to the error
                    let error_msg = format!("In function '{}', statement #{}: {}", 
                                         function.name, i + 1, e);
                    self.variables.pop();
                    self.current_function = previous_function;
                    return Err(error_msg);
                }
            }
        }

        self.variables.pop();
        self.current_function = previous_function;
        Ok(return_value)
    }

    fn eval_statement(&mut self, stmt: &Statement) -> Result<Option<Value>, String> {
        match stmt {
            Statement::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.set_variable(name, val);
                Ok(None)
            }

            Statement::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.assign_variable(name, val)?;
                Ok(None)
            }

            Statement::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", val);
                Ok(None)
            }

            Statement::ExprStmt(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }

            Statement::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(Some(val))
            }

            Statement::If { condition, then_branch, else_branch } => {
                let cond = self.eval_expr(condition)?;
                if let Value::Bool(true) = cond {
                    for stmt in then_branch {
                        if let Some(ret) = self.eval_statement(stmt)? {
                            return Ok(Some(ret));
                        }
                    }
                } else if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        if let Some(ret) = self.eval_statement(stmt)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }

            Statement::While { condition, body } => {
                // Prevent infinite loops with a reasonable limit
                let max_iterations = 100000;
                let mut iterations = 0;
                
                while let Value::Bool(true) = self.eval_expr(condition)? {
                    iterations += 1;
                    if iterations > max_iterations {
                        return Err(format!("Possible infinite loop detected: exceeded {} iterations", max_iterations));
                    }
                    
                    for stmt in body {
                        if let Some(ret) = self.eval_statement(stmt)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }

            Statement::For { init, condition, update, body } => {
                // Create a new scope for the for loop variables
                self.variables.push(HashMap::new());
                
                // Initialize
                if let Some(init_stmt) = init {
                    self.eval_statement(init_stmt)?;
                }
                
                // Prevent infinite loops with a reasonable limit
                let max_iterations = 100000;
                let mut iterations = 0;
                
                // Check condition and run body
                while condition.is_none() || 
                      matches!(self.eval_expr(condition.as_ref().unwrap())?, Value::Bool(true)) 
                {
                    iterations += 1;
                    if iterations > max_iterations {
                        self.variables.pop();
                        return Err(format!("Possible infinite loop detected: exceeded {} iterations", max_iterations));
                    }
                    
                    // Run body
                    for stmt in body {
                        if let Some(ret) = self.eval_statement(stmt)? {
                            self.variables.pop();
                            return Ok(Some(ret));
                        }
                    }
                    
                    // Update
                    if let Some(update_stmt) = update {
                        self.eval_statement(update_stmt)?;
                    }
                    
                    // If no condition, run body only once
                    if condition.is_none() {
                        break;
                    }
                }
                
                self.variables.pop();
                Ok(None)
            }

            Statement::FunctionDef { .. } => Ok(None), // Already handled in `run`

            Statement::Expression(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }

            Statement::Block(statements) => {
                self.variables.push(HashMap::new()); // new block scope
                for stmt in statements {
                    if let Some(val) = self.eval_statement(stmt)? {
                        self.variables.pop();
                        return Ok(Some(val));
                    }
                }
                self.variables.pop();
                Ok(None)
            }
            
            Statement::Import(_) => Ok(None), // Already handled in `run`
            
            Statement::Try { body, catch } => {
                // Execute try block
                let try_result = (|| -> Result<Option<Value>, String> {
                    for stmt in body {
                        if let Some(val) = self.eval_statement(stmt)? {
                            return Ok(Some(val));
                        }
                    }
                    Ok(None)
                })();
                
                // If try block had an error, execute catch block
                match try_result {
                    Ok(val) => Ok(val),
                    Err(_) => {
                        // Execute catch block
                        for stmt in catch {
                            if let Some(val) = self.eval_statement(stmt)? {
                                return Ok(Some(val));
                            }
                        }
                        Ok(None)
                    }
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::Float(f) => Ok(Value::Float(*f)),
            Expression::Bool(b) => Ok(Value::Bool(*b)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            
            Expression::Identifier(name) => {
                self.get_variable(name).ok_or(format!("Undefined variable '{}'", name))
            }
            
            Expression::Binary { left, op, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary_op(&left_val, op, &right_val)
            }
            
            Expression::Call { callee, arguments } => {
                let args = arguments
                    .iter()
                    .map(|a| self.eval_expr(a))
                    .collect::<Result<Vec<_>, _>>()?;
                
                if let Expression::Identifier(name) = &**callee {
                    self.call_function(name, args)
                } else {
                    // Handle closure calls
                    let callee_val = self.eval_expr(callee)?;
                    if let Value::Closure { params, body, env } = callee_val {
                        self.call_closure(params, body, env, args)
                    } else {
                        Err("Callee must be a function identifier or closure".into())
                    }
                }
            }
            
            Expression::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                match (op, &val) {
                    (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
                    (UnaryOp::Negate, Value::Float(f)) => Ok(Value::Float(-f)),
                    (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    _ => Err(format!("Unsupported unary operator '{:?}' for value {:?}", op, val)),
                }
            }
            
            Expression::Array { elements } => {
                let values = elements
                    .iter()
                    .map(|e| self.eval_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::Array(values))
            }
            
            Expression::ArrayAccess { array, index } => {
                let array_val = self.eval_expr(array)?;
                let index_val = self.eval_expr(index)?;
                
                match (array_val, index_val) {
                    (Value::Array(elements), Value::Number(i)) => {
                        if i < 0 {
                            return Err("Array index cannot be negative".to_string());
                        }
                        
                        let i = i as usize;
                        if i >= elements.len() {
                            return Err(format!("Array index out of bounds: {} (array length: {})", 
                                           i, elements.len()));
                        }
                        
                        Ok(elements[i].clone())
                    },
                    (Value::String(s), Value::Number(i)) => {
                        if i < 0 {
                            return Err("String index cannot be negative".to_string());
                        }
                        
                        let i = i as usize;
                        let chars: Vec<char> = s.chars().collect();
                        
                        if i >= chars.len() {
                            return Err(format!("String index out of bounds: {} (string length: {})", 
                                           i, chars.len()));
                        }
                        
                        Ok(Value::String(chars[i].to_string()))
                    },
                    (non_array, _) => Err(format!("Cannot index non-array type: {:?}", non_array)),
                }
            }
            
            Expression::Closure { params, body } => {
                // Capture the current environment
                let env = self.variables.clone();
                Ok(Value::Closure {
                    params: params.clone(),
                    body: body.clone(),
                    env,
                })
            }
        }
    }
    
    fn call_closure(&mut self, 
                   params: Vec<String>, 
                   body: Vec<Statement>, 
                   captured_env: Environment,
                   args: Vec<Value>) -> Result<Value, String> {
        // Save current environment
        let previous_env = std::mem::replace(&mut self.variables, captured_env);
        let previous_function = self.current_function.clone();
        self.current_function = Some("<closure>".to_string());
        
        if args.len() != params.len() {
            self.variables = previous_env;
            self.current_function = previous_function;
            return Err(format!(
                "Closure expects {} args, got {}",
                params.len(), args.len()
            ));
        }

        // Create new scope for arguments
        let mut scope = HashMap::new();
        for (param, arg) in params.iter().zip(args) {
            scope.insert(param.clone(), arg);
        }
        self.variables.push(scope);

        let mut return_value = Value::Void;

        // Execute body
        for (i, stmt) in body.iter().enumerate() {
            match self.eval_statement(stmt) {
                Ok(Some(val)) => {
                    return_value = val;
                    break;
                }
                Ok(None) => {}
                Err(e) => {
                    let error_msg = format!("In closure, statement #{}: {}", i + 1, e);
                    self.variables.pop();
                    self.variables = previous_env;
                    self.current_function = previous_function;
                    return Err(error_msg);
                }
            }
        }

        self.variables.pop();
        self.variables = previous_env;
        self.current_function = previous_function;
        
        Ok(return_value)
    }
    
    fn eval_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match op {
                BinaryOp::Add => Ok(Value::Number(l + r)),
                BinaryOp::Subtract => Ok(Value::Number(l - r)),
                BinaryOp::Multiply => Ok(Value::Number(l * r)),
                BinaryOp::Divide => {
                    if *r == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Number(l / r))
                    }
                },
                BinaryOp::Modulo => {
                    if *r == 0 {
                        Err("Modulo by zero".to_string())
                    } else {
                        Ok(Value::Number(l % r))
                    }
                },
                BinaryOp::Equal => Ok(Value::Bool(l == r)),
                BinaryOp::NotEqual => Ok(Value::Bool(l != r)),
                BinaryOp::LessThan => Ok(Value::Bool(l < r)),
                BinaryOp::GreaterThan => Ok(Value::Bool(l > r)),
                BinaryOp::LessThanOrEqual => Ok(Value::Bool(l <= r)),
                BinaryOp::GreaterThanOrEqual => Ok(Value::Bool(l >= r)),
                _ => Err(format!("Unsupported binary operator '{:?}' for numbers", op)),
            },
            
            (Value::Float(l), Value::Float(r)) => match op {
                BinaryOp::Add => Ok(Value::Float(l + r)),
                BinaryOp::Subtract => Ok(Value::Float(l - r)),
                BinaryOp::Multiply => Ok(Value::Float(l * r)),
                BinaryOp::Divide => {
                    if *r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Float(l / r))
                    }
                },
                BinaryOp::Equal => Ok(Value::Bool(l == r)),
                BinaryOp::NotEqual => Ok(Value::Bool(l != r)),
                BinaryOp::LessThan => Ok(Value::Bool(l < r)),
                BinaryOp::GreaterThan => Ok(Value::Bool(l > r)),
                BinaryOp::LessThanOrEqual => Ok(Value::Bool(l <= r)),
                BinaryOp::GreaterThanOrEqual => Ok(Value::Bool(l >= r)),
                _ => Err(format!("Unsupported binary operator '{:?}' for floats", op)),
            },
            
            // Mixed number and float operations
            (Value::Number(l), Value::Float(r)) => {
                let left_float = *l as f64;
                match op {
                    BinaryOp::Add => Ok(Value::Float(left_float + r)),
                    BinaryOp::Subtract => Ok(Value::Float(left_float - r)),
                    BinaryOp::Multiply => Ok(Value::Float(left_float * r)),
                    BinaryOp::Divide => {
                        if *r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Float(left_float / r))
                        }
                    },
                    BinaryOp::Equal => Ok(Value::Bool((left_float - r).abs() < f64::EPSILON)),
                    BinaryOp::NotEqual => Ok(Value::Bool((left_float - r).abs() >= f64::EPSILON)),
                    BinaryOp::LessThan => Ok(Value::Bool(left_float < *r)),
                    BinaryOp::GreaterThan => Ok(Value::Bool(left_float > *r)),
                    BinaryOp::LessThanOrEqual => Ok(Value::Bool(left_float <= *r)),
                    BinaryOp::GreaterThanOrEqual => Ok(Value::Bool(left_float >= *r)),
                    _ => Err(format!("Unsupported binary operator '{:?}' for mixed number types", op)),
                }
            },
            
            (Value::Float(l), Value::Number(r)) => {
                let right_float = *r as f64;
                match op {
                    BinaryOp::Add => Ok(Value::Float(l + right_float)),
                    BinaryOp::Subtract => Ok(Value::Float(l - right_float)),
                    BinaryOp::Multiply => Ok(Value::Float(l * right_float)),
                    BinaryOp::Divide => {
                        if *r == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Float(l / right_float))
                        }
                    },
                    BinaryOp::Equal => Ok(Value::Bool((l - right_float).abs() < f64::EPSILON)),
                    BinaryOp::NotEqual => Ok(Value::Bool((l - right_float).abs() >= f64::EPSILON)),
                    BinaryOp::LessThan => Ok(Value::Bool(*l < right_float)),
                    BinaryOp::GreaterThan => Ok(Value::Bool(*l > right_float)),
                    BinaryOp::LessThanOrEqual => Ok(Value::Bool(*l <= right_float)),
                    BinaryOp::GreaterThanOrEqual => Ok(Value::Bool(*l >= right_float)),
                    _ => Err(format!("Unsupported binary operator '{:?}' for mixed number types", op)),
                }
            },
            
            (Value::Bool(l), Value::Bool(r)) => match op {
                BinaryOp::And => Ok(Value::Bool(*l && *r)),
                BinaryOp::Or => Ok(Value::Bool(*l || *r)),
                BinaryOp::Equal => Ok(Value::Bool(l == r)),
                BinaryOp::NotEqual => Ok(Value::Bool(l != r)),
                _ => Err(format!("Unsupported operator '{:?}' for booleans", op)),
            },
            
            (Value::String(l), Value::String(r)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                BinaryOp::Equal => Ok(Value::Bool(l == r)),
                BinaryOp::NotEqual => Ok(Value::Bool(l != r)),
                _ => Err(format!("Unsupported operator '{:?}' for strings", op)),
            },
            
            (Value::String(l), Value::Number(r)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(format!("Unsupported operator '{:?}' between string and number", op)),
            },
            
            (Value::String(l), Value::Float(r)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(format!("Unsupported operator '{:?}' between string and float", op)),
            },
            
            (Value::Number(l), Value::String(r)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(format!("Unsupported operator '{:?}' between number and string", op)),
            },
            
            (Value::Float(l), Value::String(r)) => match op {
                BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(format!("Unsupported operator '{:?}' between float and string", op)),
            },
            
            _ => Err(format!("Type mismatch in binary operation: {:?} {:?} {:?}", left, op, right)),
        }
    }

    fn set_variable(&mut self, name: &str, val: Value) {
        if let Some(scope) = self.variables.last_mut() {
            scope.insert(name.to_string(), val);
        }
    }

    fn assign_variable(&mut self, name: &str, val: Value) -> Result<(), String> {
        for scope in self.variables.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return Ok(());
            }
        }
        Err(format!("Cannot assign to undefined variable '{}'", name))
    }

    fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in self.variables.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }
}