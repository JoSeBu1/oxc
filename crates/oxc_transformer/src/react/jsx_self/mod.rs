use std::rc::Rc;

use oxc_ast::ast::*;
use oxc_span::SPAN;

use crate::context::Ctx;

/// [plugin-transform-react-jsx-self](https://babeljs.io/docs/babel-plugin-transform-react-jsx-self)
///
/// This plugin is included in `preset-react` and only enabled in development mode.
///
/// ## Example
///
/// In: `<sometag />`
/// Out: `<sometag __self={this} />`
///
/// TODO:
/// *. Omit adding `this` in some conditions
///    <https://github.com/babel/babel/blob/9cd048b5ad45eafd157c4f9968343e36170a66c1/packages/babel-plugin-transform-react-jsx-self/src/index.ts#L78>
#[allow(unused)]
pub struct ReactJsxSelf<'a> {
    ctx: Ctx<'a>,
}

impl<'a> ReactJsxSelf<'a> {
    pub fn new(ctx: &Ctx<'a>) -> Self {
        Self { ctx: Rc::clone(ctx) }
    }

    pub fn transform_jsx_opening_element(&self, elem: &mut JSXOpeningElement<'a>) {
        self.add_self_this_attribute(elem);
    }
}

impl<'a> ReactJsxSelf<'a> {
    /// `<div __self={this} />`
    ///       ^^^^^^^^^^^^^
    fn add_self_this_attribute(&self, elem: &mut JSXOpeningElement<'a>) {
        let name = JSXAttributeName::Identifier(JSXIdentifier::new(SPAN, "__self".into()));
        let value = {
            let jsx_expr = JSXExpression::Expression(self.ctx.ast.this_expression(SPAN));
            let container = self.ctx.ast.jsx_expression_container(SPAN, jsx_expr);
            JSXAttributeValue::ExpressionContainer(container)
        };
        let attribute = {
            let attribute = self.ctx.ast.jsx_attribute(SPAN, name, Some(value));
            JSXAttributeItem::Attribute(attribute)
        };
        elem.attributes.push(attribute);
    }
}
