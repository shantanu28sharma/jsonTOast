use crate::ast::{
    Array, Literal, LiteralValue, Node, Object, Point, Property, PropertyValue, Span,
};
use std::fmt;

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Node::Object(ref obj) => write!(f, "{:?}", obj),
            &Node::Array(ref arr) => write!(f, "{:?}", arr),
            &Node::Literal(ref lit) => write!(f, "{:?}", lit),
        }
    }
}

impl fmt::Debug for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &PropertyValue::Object(ref obj) => write!(f, "{:?}", obj),
            &PropertyValue::Array(ref arr) => write!(f, "{:?}", arr),
            &PropertyValue::Literal(ref lit) => write!(f, "{:?}", lit),
        }
    }
}

impl fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &LiteralValue::Str(ref st) => write!(f, "{:?}", st),
            &LiteralValue::Num(ref num) => write!(f, "{}", num),
            &LiteralValue::Bool(ref bl) => write!(f, "{}", bl),
            &LiteralValue::Null => write!(f, "null"),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("type", &"Object")
            .field("children", &self.children)
            .field("loc", &self.span)
            .finish()
    }
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("type", &"Property")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("loc", &self.span)
            .finish()
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("type", &"Array")
            .field("children", &self.children)
            .finish()
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("type", &"Literal")
            .field("value", &self.value)
            .finish()
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}
