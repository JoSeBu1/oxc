mod annotations;
mod namespace;

use std::rc::Rc;

use serde::Deserialize;

use oxc_allocator::Vec;
use oxc_ast::ast::*;

use crate::context::Ctx;

use self::annotations::TypeScriptAnnotations;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeScriptOptions;

/// [Preset TypeScript](https://babeljs.io/docs/babel-preset-typescript)
///
/// This preset includes the following plugins:
///
/// * [transform-typescript](https://babeljs.io/docs/babel-plugin-transform-typescript)
///
/// This plugin adds support for the types syntax used by the TypeScript programming language.
/// However, this plugin does not add the ability to type-check the JavaScript passed to it.
/// For that, you will need to install and set up TypeScript.
///
/// Note that although the TypeScript compiler tsc actively supports certain JavaScript proposals such as optional chaining (?.),
/// nullish coalescing (??) and class properties (this.#x), this preset does not include these features
/// because they are not the types syntax available in TypeScript only.
/// We recommend using preset-env with preset-typescript if you want to transpile these features.
///
/// This plugin is included in `preset-typescript`.
///
/// ## Example
///
/// In:  `const x: number = 0;`
/// Out: `const x = 0;`
#[allow(unused)]
pub struct TypeScript<'a> {
    options: Rc<TypeScriptOptions>,
    ctx: Ctx<'a>,

    annotations: TypeScriptAnnotations<'a>,
}

impl<'a> TypeScript<'a> {
    pub fn new(options: TypeScriptOptions, ctx: &Ctx<'a>) -> Self {
        let options = Rc::new(options);

        Self {
            annotations: TypeScriptAnnotations::new(&options, ctx),
            options,
            ctx: Rc::clone(ctx),
        }
    }
}

// Transforms
impl<'a> TypeScript<'a> {
    pub fn transform_program_on_exit(&self, program: &mut Program<'a>) {
        self.annotations.transform_program_on_exit(program);
    }

    pub fn transform_arrow_expression(&mut self, expr: &mut ArrowFunctionExpression<'a>) {
        self.annotations.transform_arrow_expression(expr);
    }

    pub fn transform_binding_pattern(&mut self, pat: &mut BindingPattern<'a>) {
        self.annotations.transform_binding_pattern(pat);
    }

    pub fn transform_call_expression(&mut self, expr: &mut CallExpression<'a>) {
        self.annotations.transform_call_expression(expr);
    }

    pub fn transform_class(&mut self, class: &mut Class<'a>) {
        self.annotations.transform_class(class);
    }

    pub fn transform_class_body(&mut self, body: &mut ClassBody<'a>) {
        self.annotations.transform_class_body(body);
    }

    pub fn transform_export_named_declaration(&mut self, decl: &mut ExportNamedDeclaration<'a>) {
        self.annotations.transform_export_named_declaration(decl);
    }

    pub fn transform_expression(&mut self, expr: &mut Expression<'a>) {
        self.annotations.transform_expression(expr);
    }

    pub fn transform_formal_parameter(&mut self, param: &mut FormalParameter<'a>) {
        self.annotations.transform_formal_parameter(param);
    }

    pub fn transform_function(
        &mut self,
        func: &mut Function<'a>,
        flags: Option<oxc_semantic::ScopeFlags>,
    ) {
        self.annotations.transform_function(func, flags);
    }

    pub fn transform_import_declaration(&mut self, decl: &mut ImportDeclaration<'a>) {
        self.annotations.transform_import_declaration(decl);
    }

    pub fn transform_jsx_opening_element(&mut self, elem: &mut JSXOpeningElement<'a>) {
        self.annotations.transform_jsx_opening_element(elem);
    }

    pub fn transform_method_definition(&mut self, def: &mut MethodDefinition<'a>) {
        self.annotations.transform_method_definition(def);
    }

    pub fn transform_new_expression(&mut self, expr: &mut NewExpression<'a>) {
        self.annotations.transform_new_expression(expr);
    }

    pub fn transform_property_definition(&mut self, def: &mut PropertyDefinition<'a>) {
        self.annotations.transform_property_definition(def);
    }

    pub fn transform_statements(&mut self, stmts: &mut Vec<'a, Statement<'a>>) {
        self.transform_statements_for_namespace(stmts);
    }

    pub fn transform_statements_on_exit(&mut self, stmts: &mut Vec<'a, Statement<'a>>) {
        self.annotations.transform_statements_on_exit(stmts);
    }

    pub fn transform_tagged_template_expression(
        &mut self,
        expr: &mut TaggedTemplateExpression<'a>,
    ) {
        self.annotations.transform_tagged_template_expression(expr);
    }
}
