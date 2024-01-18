use std::{
	fmt::{self, Debug, Display, Formatter},
	rc::Rc,
};

use dyn_clone::{clone_trait_object, DynClone};
use parser::ast::{Boolean, Function, Integer, Literal, StringT};
use strum::EnumDiscriminants;

use crate::environment::Frame;

pub struct ArgumentValues(pub Vec<Value>);

pub trait BuiltinFunction: DynClone {
	fn call(&mut self, args: ArgumentValues) -> Value;
}

clone_trait_object!(BuiltinFunction);

pub enum FunctionValue {
	Builtin(Box<dyn BuiltinFunction + Send + Sync>),
	Closure { body: Function, capture: Rc<Frame> },
}

impl Debug for FunctionValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Builtin(_) => write!(f, "[BUILTIN FUNCTION]"),
			Self::Closure { body, capture } => f
				.debug_tuple("Lambda")
				.field(body)
				.finish(),
		}
	}
}

impl Clone for FunctionValue {
	fn clone(&self) -> Self {
		match self {
			Self::Builtin(arg0) => Self::Builtin(arg0.clone()),
			Self::Closure { body, capture } => Self::Closure {
				body: body.clone(),
				capture: Rc::clone(&capture),
			},
		}
	}
}

impl PartialEq for FunctionValue {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Builtin(_), Self::Builtin(_)) => unimplemented!(),
			(
				Self::Closure { body: b1, .. },
				Self::Closure { body: b2, .. },
			) => b1 == b2,
			_ => false,
		}
	}
}

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(ValueType))]
pub enum Value {
	Nil,
	Boolean(bool),
	Integer(i32),
	String(String),
	Array(Vec<Value>),
	Function(FunctionValue),
}

impl Value {
	#[allow(unused)]
	fn from_literal(ex: Literal) -> Self {
		match ex {
			Literal::Nil(_) => Self::Nil,
			Literal::Boolean(Boolean { value, .. }) => Self::Boolean(value),
			Literal::Integer(Integer { value, .. }) => Self::Integer(value),
			Literal::String(StringT { value, .. }) => Self::String(value),
		}
	}

	pub fn to_debug(&self) -> String {
		match self {
			Value::Nil => "nil".to_owned(),
			Value::Boolean(v) =>
				if *v {
					"true".to_owned()
				} else {
					"false".to_owned()
				},
			Value::Integer(v) => format!("{v}"),
			Value::String(v) => format!(r#""{}""#, v),
			Value::Array(v) => format!(
				"[{}]",
				v.iter()
					.map(|e| e.to_debug())
					.collect::<Vec<_>>()
					.join(", ")
			),
			Value::Function(_) => "[FUNCTION]".to_owned(),
		}
	}

	pub fn get_type(&self) -> ValueType { self.into() }
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Value::Nil => "nil".to_owned(),
			Value::Boolean(i) => i.to_string(),
			Value::Integer(i) => i.to_string(),
			Value::String(i) => i.to_string(),
			Value::Array(i) => format!("{i:?}"),
			Value::Function(_) => "FUNCTION".to_owned(),
		})
	}
}

impl ValueType {
	pub fn type_name(&self) -> &'static str {
		match self {
			ValueType::Nil => "nil",
			ValueType::Boolean => "bool",
			ValueType::Integer => "number",
			ValueType::String => "string",
			ValueType::Array => "array",
			ValueType::Function => "function",
		}
	}
}

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(FlatValueType))]
pub enum FlatValue {
	Nil,
	Boolean(bool),
	Integer(i32),
	String(String),
	Array(Vec<Value>),
	Function,
}

impl FlatValue {
	pub fn to_debug(&self) -> String {
		match self {
			FlatValue::Nil => "nil".to_owned(),
			FlatValue::Boolean(v) =>
				if *v {
					"true".to_owned()
				} else {
					"false".to_owned()
				},
			FlatValue::Integer(v) => format!("{v}"),
			FlatValue::String(v) => format!(r#""{}""#, v),
			FlatValue::Array(v) => format!(
				"[{}]",
				v.iter()
					.map(|e| e.to_debug())
					.collect::<Vec<_>>()
					.join(", ")
			),
			FlatValue::Function => "[FUNCTION]".to_owned(),
		}
	}

	pub fn get_type(&self) -> FlatValueType { self.into() }
}

impl From<Value> for FlatValue {
	fn from(value: Value) -> Self {
		match value {
			Value::Nil => Self::Nil,
			Value::Boolean(v) => Self::Boolean(v),
			Value::Integer(v) => Self::Integer(v),
			Value::String(v) => Self::String(v),
			Value::Array(v) => Self::Array(v),
			Value::Function(v) => Self::Function,
		}
	}
}

impl FlatValueType {
	pub fn type_name(&self) -> &'static str {
		match self {
			FlatValueType::Nil => "nil",
			FlatValueType::Boolean => "bool",
			FlatValueType::Integer => "number",
			FlatValueType::String => "string",
			FlatValueType::Array => "array",
			FlatValueType::Function => "function",
		}
	}
}
