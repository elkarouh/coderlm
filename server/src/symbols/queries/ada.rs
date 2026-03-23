use super::{LanguageConfig, TestPattern};

/// Tree-sitter query for Ada symbol extraction.
/// Ada constructs mapped to standard symbol kinds:
/// - procedure/function specifications/bodies → Function
/// - package declarations/bodies → Module
/// - full_type_declaration with record_type_definition → Struct
/// - enumeration_type_definition → Enum
/// - other type declarations → Type
pub const SYMBOLS_QUERY: &str = r#"
; Procedure specifications (declarations in package specs)
(procedure_specification
  name: (identifier) @function.name) @function.def

(procedure_specification
  name: (selected_component
    selector_name: (identifier) @function.name)) @function.def

; Function specifications (declarations in package specs)
(function_specification
  name: (identifier) @function.name) @function.def

(function_specification
  name: (selected_component
    selector_name: (identifier) @function.name)) @function.def

; Subprogram bodies (full procedure/function definitions)
; Note: procedure/function_specification are unnamed children of subprogram_body
(subprogram_body
  (procedure_specification
    name: (identifier) @function.name)) @function.def

(subprogram_body
  (procedure_specification
    name: (selected_component
      selector_name: (identifier) @function.name))) @function.def

(subprogram_body
  (function_specification
    name: (identifier) @function.name)) @function.def

(subprogram_body
  (function_specification
    name: (selected_component
      selector_name: (identifier) @function.name))) @function.def

; Package declarations (specifications)
(package_declaration
  name: (identifier) @mod.name) @mod.def

(package_declaration
  name: (selected_component
    selector_name: (identifier) @mod.name)) @mod.def

; Package bodies
(package_body
  name: (identifier) @mod.name) @mod.def

(package_body
  name: (selected_component
    selector_name: (identifier) @mod.name)) @mod.def

; Record type declarations → Struct
; Note: name and type_definition are unnamed children of full_type_declaration
(full_type_declaration
  (identifier) @struct.name
  (record_type_definition)) @struct.def

; Enumeration type declarations → Enum
(full_type_declaration
  (identifier) @enum.name
  (enumeration_type_definition)) @enum.def

; Other type declarations (derived, array, etc.) → Type
(full_type_declaration
  (identifier) @type.name) @type.def

; Subtype declarations (name is an unnamed child)
(subtype_declaration
  (identifier) @type.name) @type.def

; Private type declarations (name is an unnamed child)
(private_type_declaration
  (identifier) @type.name) @type.def

; Number declarations (name is an unnamed child)
(number_declaration
  (identifier) @const.name) @const.def

; Object declarations with constant keyword
(object_declaration
  name: (identifier) @const.name
  "constant") @const.def

; Exception declarations (name is an unnamed child)
(exception_declaration
  (identifier) @const.name) @const.def

; Generic instantiations
(generic_instantiation
  name: (identifier) @type.name) @type.def
"#;

/// Tree-sitter query for Ada call expressions.
/// Captures procedure and function calls.
pub const CALLERS_QUERY: &str = r#"
; Procedure call statements
(procedure_call_statement
  name: (identifier) @callee)

(procedure_call_statement
  name: (selected_component
    selector_name: (identifier) @callee))

; Function calls (in expressions)
(function_call
  name: (identifier) @callee)

(function_call
  name: (selected_component
    selector_name: (identifier) @callee))
"#;

/// Tree-sitter query for Ada variable declarations.
/// Captures local object declarations and parameters.
pub const VARIABLES_QUERY: &str = r#"
; Object declarations (variables) — name is a named field
(object_declaration
  name: (identifier) @var.name)

; Parameter specifications — name is an unnamed child
(parameter_specification
  (identifier) @var.name)

; Discriminant specifications — name is an unnamed child
(discriminant_specification
  (identifier) @var.name)

; Loop parameter specification — name is an unnamed child
(loop_parameter_specification
  (identifier) @var.name)

; Iterator specification — name is an unnamed child
(iterator_specification
  (identifier) @var.name)
"#;

pub fn config() -> LanguageConfig {
    LanguageConfig {
        language: tree_sitter_ada::LANGUAGE.into(),
        symbols_query: SYMBOLS_QUERY,
        callers_query: CALLERS_QUERY,
        variables_query: VARIABLES_QUERY,
        // Ada uses "AUnit" test framework with Test_ prefix convention
        test_patterns: vec![TestPattern::FunctionPrefix("Test_")],
    }
}
