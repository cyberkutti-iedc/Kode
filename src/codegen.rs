use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, GlobalValue};
use inkwell::OptimizationLevel;
use inkwell::targets::{CodeModel, RelocMode, InitializationConfig, Target, TargetMachine};
use inkwell::AddressSpace;
use inkwell::passes::PassManager;
use std::path::Path;
use crate::ast::{Statement, Expression};

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    main_fn: Option<FunctionValue<'ctx>>,
    pass_manager: PassManager<FunctionValue<'ctx>>
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("kode_program");
        let builder = context.create_builder();
        let pass_manager = PassManager::create(&module);

        // Create printf declaration
        let i32_type = context.i32_type();
        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        module.add_function("printf", printf_type, None);

        CodeGenerator {
            context,
            module,
            builder,
            main_fn: None,
            pass_manager
        }
    }

    pub fn generate(&mut self, ast: &[Statement], optimize: bool) -> Result<(), String> {
        // Create main function
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);
        let entry = self.context.append_basic_block(main_fn, "entry");
        
        self.builder.position_at_end(entry);
        self.main_fn = Some(main_fn);

        // Generate code for each statement
        for stmt in ast {
            self.generate_statement(stmt)?;
        }

        // Add return 0
        let _ = self.builder.build_return(Some(&i32_type.const_int(0, false)));

        // Verify the module
        if self.module.verify().is_err() {
            return Err("Invalid LLVM IR generated".to_string());
        }

        // Optimize if requested
        if optimize {
            let pass_manager = PassManager::create(&self.module);
            pass_manager.add_instruction_combining_pass();
            pass_manager.add_reassociate_pass();
            pass_manager.add_gvn_pass();
            pass_manager.add_cfg_simplification_pass();
            pass_manager.initialize();

            for function in self.module.get_functions() {
                pass_manager.run_on(&function);
            }
        }

        Ok(())
    }

    fn generate_statement(&self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Print(expr) => self.generate_print(expr),
            Statement::FunctionDef { name, params: _, body, .. } => {
                if name == "app" || name == "main" {
                    // Generate code for function body
                    for stmt in body {
                        self.generate_statement(stmt)?;
                    }
                }
                Ok(())
            }
            _ => Ok(()) // Ignore other statements for now
        }
    }

    fn generate_print(&self, expr: &Expression) -> Result<(), String> {
        let printf = self.module.get_function("printf").unwrap();

        match expr {
            Expression::String(s) => {
                let format_str = self.builder.build_global_string_ptr(&format!("{}\n", s), "str")
                    .map_err(|e| format!("Failed to build string: {:?}", e))?;
                let _ = self.builder.build_call(
                    printf,
                    &[format_str.as_pointer_value().into()],
                    "printf_call"
                );
                Ok(())
            }
            Expression::Number(n) => {
                let format_str = self.builder.build_global_string_ptr("%d\n", "int_fmt")
                    .map_err(|e| format!("Failed to build format string: {:?}", e))?;
                let value = self.context.i32_type().const_int(*n as u64, false);
                let _ = self.builder.build_call(
                    printf,
                    &[format_str.as_pointer_value().into(), value.into()],
                    "printf_call"
                );
                Ok(())
            }
            _ => Err("Unsupported print expression type".to_string())
        }
    }

    pub fn write_object_file(&self, path: &Path) -> Result<(), String> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| format!("Failed to initialize target: {}", e))?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|e| format!("Failed to get target: {}", e))?;

            let machine = target.create_target_machine(
                &triple,
                "generic",
                "",
                if self.module.get_functions().count() > 0 {
                    OptimizationLevel::Default
                } else {
                    OptimizationLevel::None
                },
                RelocMode::Static,  // Change to static relocation
                CodeModel::Small,
            ).ok_or_else(|| "Failed to create target machine".to_string())?;
        
        machine.write_to_file(&self.module, inkwell::targets::FileType::Object, path)
            .map_err(|e| format!("Failed to write object file: {}", e))
    }

}