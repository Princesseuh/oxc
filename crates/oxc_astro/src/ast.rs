//! [Astro](https://github.com/withastro/)

// NB: `#[span]`, `#[scope(...)]`,`#[visit(...)]` and `#[generate_derive(...)]` do NOT do anything to the code.
// They are purely markers for codegen used in `tasks/ast_tools` and `crates/oxc_traverse/scripts`. See docs in those crates.
// Read [`macro@oxc_ast_macros::ast`] for more information.

// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]

use oxc_allocator::{Box, CloneIn, Vec};
use oxc_ast_macros::ast;
use oxc_span::{Atom, GetSpan, GetSpanMut, Span};
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use tsify::Tsify;

use super::{inherit_variants, js::*};

// Astro Program
//
// ## Examples
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct AstroProgram<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// Frontmatter (aka "component script")
    pub frontmatter: Option<Box<'a, AstroFrontMatter<'a>>>,
}

// Astro Element
//
// ## Examples
//
/// ```astro
/// --- // <- frontmatter (we don't need an opening/closing_element)
/// const title = "Hello, Astro!"; // <- frontmatter content
/// ---
/// ````
///
/// ```astro
/// <Foo>        // <- opening_element
///   some text  // <- children
/// </Foo>       // <- closing_element
/// ```
///
/// ```astro
/// <Foo />     // <- opening_element, no closing_element
/// ```
///
/// See [Astro Syntax](https://docs.astro.build/en/basics/astro-syntax/).
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct AstroElement<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// Opening tag of the element.
    pub opening_element: Box<'a, AstroOpeningElement<'a>>,
    /// Closing tag of the element. Will be [`None`] for self-closing tags.
    pub closing_element: Option<Box<'a, AstroClosingElement<'a>>>,
    /// Children of the element. This can be text, other elements, or expressions.
    pub children: Vec<'a, AstroChild<'a>>,
}

// Astro Frontmatter (aka "component script")
//
// ## Examples
//
// ```astro
// ---
// title: "Hello, Astro!"
// ---
// ```
//
// See [Astro Frontmatter](https://docs.astro.build/en/basics/astro-components/#the-component-script).
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct AstroFrontMatter<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// Contenu du frontmatter (JavaScript/TypeScript)
    pub content: Vec<'a, AstroChild<'a>>,
}

// Astro Opening Element
//
// ## Examples
//
// ```astro
// <Foo>        // <- opening_element
//   some text  // <- children
// </Foo>       // <- closing_element
// ```
//
// ```astro
// <Foo />     // <- opening_element, no closing_element
// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct AstroOpeningElement<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// Is this tag self-closing?
    ///
    /// ## Examples
    /// ```astro
    /// <Foo />  // <- self_closing = true
    /// <Foo>    // <- self_closing = false
    /// ```
    pub self_closing: bool,
    pub name: AstroElementName<'a>,
    /// List of Astro attributes. These become props in Astro components.
    pub attributes: Vec<'a, AstroAttributeItem<'a>>,
    /// Type parameters for generic Astro elements.
    pub type_parameters: Option<Box<'a, TSTypeParameterInstantiation<'a>>>,
}

// Astro Closing Element
//
// Closing tag in an [`AstroElement`]. Self-closing tags do not have closing elements.
//
// ## Example
//
// ```astro
// <Foo>Hello, Astro!</Foo>
// //                  ^^^ name
// <Bar /> // <- no closing element
// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroClosingElement<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub name: AstroElementName<'a>,
}

// Astro Fragment
//
// A fragment written with the special `<></>` syntax. These are typical in Astro as well.
//
// See [Astro Fragments](https://docs.astro.build/en/basics/astro-syntax/#fragments)
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct AstroFragment<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// `<>`
    pub opening_fragment: AstroOpeningFragment,
    /// `</>`
    pub closing_fragment: AstroClosingFragment,
    /// Elements inside the fragment.
    pub children: Vec<'a, AstroChild<'a>>,
}

/// Astro Opening Fragment (`<>`)
#[ast]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroOpeningFragment {
    #[serde(flatten)]
    pub span: Span,
}

/// Astro Closing Fragment (`</>`)
#[ast]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroClosingFragment {
    #[serde(flatten)]
    pub span: Span,
}

/// Astro Element Name
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(untagged)]
pub enum AstroElementName<'a> {
    /// `<div />`
    Identifier(Box<'a, AstroIdentifier<'a>>) = 0,
    /// `<Component />`
    IdentifierReference(Box<'a, IdentifierReference<'a>>) = 1,
    /// `<Namespace:Component />`
    NamespacedName(Box<'a, AstroNamespacedName<'a>>) = 2,
    /// `<Module.Component />`
    MemberExpression(Box<'a, AstroMemberExpression<'a>>) = 3,
}

/// Astro Namespaced Name
///
/// ## Example
///
/// ```astro
/// <Namespace:Component />
/// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroNamespacedName<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// Namespace portion of the name, e.g. `Namespace` in `<Namespace:Component />`
    pub namespace: AstroIdentifier<'a>,
    /// Name portion of the name, e.g. `Component` in `<Namespace:Component />`
    pub property: AstroIdentifier<'a>,
}

/// Astro Member Expression
///
/// Used in [`AstroElementName`]. Multiple member expressions may be chained together. In this case,
/// [`object`] will be a [`member expression`].
///
/// ## Example
///
/// ```astro
/// <Module.Component />
/// <Library.UI.Button />
/// ```
///
/// [`object`]: AstroMemberExpression::object
/// [`member expression`]: AstroMemberExpressionObject::MemberExpression
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroMemberExpression<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The object being accessed. This is everything before the last `.`.
    pub object: AstroMemberExpressionObject<'a>,
    /// The property being accessed. This is everything after the last `.`.
    pub property: AstroIdentifier<'a>,
}

#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(untagged)]
pub enum AstroMemberExpressionObject<'a> {
    IdentifierReference(Box<'a, IdentifierReference<'a>>) = 1,
    MemberExpression(Box<'a, AstroMemberExpression<'a>>) = 2,
}

/// Astro Identifier
///
/// Similar to [`IdentifierName`], but used in Astro elements.
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroIdentifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The name of the identifier.
    pub name: Atom<'a>,
}

/// Astro Child
///
/// Part of a [`AstroElement`].
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(untagged)]
pub enum AstroChild<'a> {
    /// `<Foo>Some Text</Foo>`
    Text(Box<'a, AstroText<'a>>) = 0,
    /// `<Foo><Child /></Foo>`
    Element(Box<'a, AstroElement<'a>>) = 1,
    /// `<Foo><></></Foo>`
    Fragment(Box<'a, AstroFragment<'a>>) = 2,
    /// `<Foo>{expression}</Foo>`
    ExpressionContainer(Box<'a, AstroExpressionContainer<'a>>) = 3,
    /// `<Foo>{...spread}</Foo>`
    Spread(Box<'a, AstroSpreadChild<'a>>) = 4,
}

/// Astro Spread Child.
///
/// Variant of [`AstroChild`] that represents an object spread (`{...expression}`).
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroSpreadChild<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The expression being spread.
    pub expression: Expression<'a>,
}

/// Text inside an Astro element.
///
/// Not to be confused with a [`StringLiteral`].
///
/// ## Example
///
/// ```astro
/// <Foo>Some text</Foo>     // `Some Text` is a AstroText,
/// <Foo>"Some string"</Foo> // but `"Some string"` is a StringLiteral.
/// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroText<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The text content.
    pub value: Atom<'a>,
}

/// Astro Expression Container
///
/// Expression containers wrap [`AstroExpression`]s in Astro attributes and children using `{}`.
///
/// ## Example
///
/// ```astro
/// <Foo bar baz="bang" container={4}/>
///   {4}  // <- wrapped in container
/// </Foo>
/// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroExpressionContainer<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The expression inside the container.
    pub expression: AstroExpression<'a>,
}

inherit_variants! {
/// Astro Expression
///
/// Gets wrapped by a [`AstroExpressionContainer`]. Inherits variants from [`Expression`]. See [`ast`
/// module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(untagged)]
pub enum AstroExpression<'a> {
    EmptyExpression(AstroEmptyExpression) = 64,
    // `Expression` variants added here by `inherit_variants!` macro
    @inherit Expression
}
}

/// An empty Astro expression (`{}`)
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroEmptyExpression {
    #[serde(flatten)]
    pub span: Span,
}

// Astro Attributes

/// Astro Attributes
///
/// ## Example
///
/// ```astro
/// <Component foo="bar" baz={4} {...rest} />
/// //         ^^^^^^^^^ ^^^^^^^ ^^^^^^^^^
/// //             Attribute     SpreadAttribute
/// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(untagged)]
pub enum AstroAttributeItem<'a> {
    /// A `key="value"` attribute
    Attribute(Box<'a, AstroAttribute<'a>>) = 0,
    /// a `{...spread}` attribute
    SpreadAttribute(Box<'a, AstroSpreadAttribute<'a>>) = 1,
}

/// Astro Attribute
///
/// An attribute in an Astro opening tag. May or may not have a value. Part of
/// [`AstroAttributeItem`].
///
/// ## Example
///
/// ```astro
/// // `has-no-value` is an AstroAttribute with no value.
/// <Component has-no-value foo="foo" />
/// //                 name ^^^ ^^^^ value
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroAttribute<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The name of the attribute. This is a prop in Astro components.
    pub name: AstroAttributeName<'a>,
    /// The value of the attribute. This can be a string literal, an expression,
    /// or an element. Will be [`None`] for boolean-like attributes (e.g.
    /// `<button disabled />`).
    pub value: Option<AstroAttributeValue<'a>>,
}

/// Astro Spread Attribute
///
/// ## Example
/// ```astro
/// <Component {...props} />
/// //          ^^^^^^^^ argument
/// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(tag = "type")]
pub struct AstroSpreadAttribute<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub argument: Expression<'a>,
}

/// Astro Attribute Name
///
/// Part of a [`AstroAttribute`].
///
/// "Normal" attributes will be a [`AstroIdentifier`], while namespaced attributes
/// will be a [`AstroNamespacedName`].
///
/// ## Example
///
/// ```astro
/// const Foo = <Component foo="bar" />;
/// //                     ^^^ Identifier
/// const Bar = <Component foo:bar="baz" />;
/// //                     ^^^^^^^ NamespacedName
/// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(untagged)]
pub enum AstroAttributeName<'a> {
    /// An attribute name without a namespace prefix, e.g. `foo` in `foo="bar"`.
    Identifier(Box<'a, AstroIdentifier<'a>>) = 0,
    /// An attribute name with a namespace prefix, e.g. `foo:bar` in `foo:bar="baz"`.
    NamespacedName(Box<'a, AstroNamespacedName<'a>>) = 1,
}

/// Astro Attribute Value
///
/// Part of a [`AstroAttribute`].
///
/// You're most likely interested in [`StringLiteral`] and
/// [`AstroExpressionContainer`].
///
/// ## Example
///
/// ```astro
/// //                        v ExpressionContainer storing a NumericLiteral
/// <Component foo="bar" baz={4} />
/// //              ^^^ StringLiteral
///
/// // not a very common case, but it is valid syntax. Could also be a fragment.
/// <Component foo=<Element /> />
/// //             ^^^^^^^^^^^ Element
/// ```
#[ast(visit)]
#[derive(Debug, Hash)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[serde(untagged)]
pub enum AstroAttributeValue<'a> {
    StringLiteral(Box<'a, StringLiteral<'a>>) = 0,
    ExpressionContainer(Box<'a, AstroExpressionContainer<'a>>) = 1,
    Element(Box<'a, AstroElement<'a>>) = 2,
    Fragment(Box<'a, AstroFragment<'a>>) = 3,
}
