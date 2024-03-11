extern crate toolshed;

use pretty_assertions::{assert_eq, assert_ne};
use std::marker::PhantomData;
use toolshed::list::{List, UnsafeList};
use toolshed::Arena;

pub use crate::ast::node::{Node, NodeInner, OptionalLocation};
pub use crate::ast::source::*;
pub use crate::ast::statement::*;

/// Useful for boolean flags that need location information via FlagNode,
/// for example: `indexed` or `anonymous`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Flag;

pub type Identifier<'ast> = &'ast str;
pub type StringLiteral<'ast> = &'ast str;
pub type VersionLiteral<'ast> = &'ast str;

pub type FlagNode<'ast> = Node<'ast, Flag>;
pub type NodeList<'ast, T> = List<'ast, Node<'ast, T>>;
pub type SourceUnitNode<'ast> = Node<'ast, SourceUnit<'ast>>;
pub type SourceUnitList<'ast> = NodeList<'ast, SourceUnit<'ast>>;
pub type IdentifierNode<'ast> = Node<'ast, Identifier<'ast>>;
pub type IdentifierList<'ast> = NodeList<'ast, Identifier<'ast>>;
pub type StringLiteralNode<'ast> = Node<'ast, StringLiteral<'ast>>;

/// A Solidity source code parsed to an AST
pub struct Program<'ast> {
    /// `SourceUnitList<'ast>` converted to an `UnsafeList` to deal with
    /// the fact that the `Arena` on which it lives is also in this struct.
    body: UnsafeList,

    /// `Arena` on which the entire AST is allocated.
    arena: Arena,

    /// For lifetime safety :).
    _phantom: PhantomData<SourceUnitList<'ast>>,
}

impl<'ast> Program<'ast> {
    #[inline]
    pub fn new(body: UnsafeList, arena: Arena) -> Self {
        Program {
            body,
            arena,
            _phantom: PhantomData,
        }
    }

    /// Get the list of `SourceUnit`s.
    #[inline]
    pub fn body(&self) -> SourceUnitList<'ast> {
        unsafe { self.body.into_list() }
    }

    /// Get a reference to the `Arena` on which the AST is allocated.
    #[inline]
    pub fn arena(&'ast self) -> &'ast Arena {
        &self.arena
    }
}
