use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::ExprFunctionCall;

pub const OK_EXPECT_IN_CODE: &str = "Leaving `expect` in the code is discourage.";
const EXPECT: &str = "\"OptionTraitImpl::expect\"";
const OK: &str = "\"ResultTraitImpl::ok\"";

pub fn check_ok_expect_usage(
    db: &dyn SemanticGroup,
    expr_function_call: &ExprFunctionCall,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_id = expr_function_call.function;

    // println!("EXPR: {}", function_id.name(db));

    if function_id.name(db) == OK {
        println!("OK EXPR:");
        dbg!(expr_function_call.clone().path());
        diagnostics.push(PluginDiagnostic {
            stable_ptr: expr_function_call.stable_ptr.into(),
            message: OK_EXPECT_IN_CODE.to_owned(),
            severity: Severity::Warning,
        });
    }

    // if function_id.name(db) == EXPECT {
    //     println!("EXPECT EXPR:");
    //     dbg!(expr_function_call.stable_ptr);
    //     diagnostics.push(PluginDiagnostic {
    //         stable_ptr: expr_function_call.stable_ptr.into(),
    //         message: OK_EXPECT_IN_CODE.to_owned(),
    //         severity: Severity::Warning,
    //     });
    // }
}
