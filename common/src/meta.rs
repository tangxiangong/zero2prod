use serde::Serialize;

/// 自定义响应 [`AppResponse`](crate::AppResponse) 中元数据类型需要实现的 trait
pub trait Meta: Serialize {}

impl Meta for () {}

impl Meta for serde_json::Value {}
