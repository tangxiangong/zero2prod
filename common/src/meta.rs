use serde::Serialize;

/// 自定义响应 [`AppResponse`](crate::AppResponse) 中元数据类型需要实现的 trait
///
/// # Example
///
/// ```
/// use axum::http::StatusCode;
/// use serde::Serialize;
/// use common::{AppResponse, utils::Meta};
///
/// #[derive(Serialize)]
/// struct Book {
///     author: String,
/// }
///
/// #[derive(Serialize)]
/// struct BookMeta {
///     number: usize,
/// }
///
/// impl Meta for BookMeta {
///     type Item = Book;
/// }
///
/// // handler
/// async fn test() -> AppResponse<Vec<Book>, BookMeta> {
///    let data = vec![
///         Book {
///             author: "1".to_owned(),
///         },
///         Book {
///             author: "2".to_owned(),
///         },
///     ];
///     let metadata = BookMeta { number: 2 };
///     AppResponse::with_meta(StatusCode::OK, data, metadata)
/// }
/// ```
pub trait Meta: Serialize {
    type Item: Serialize;
}

impl Meta for () {
    type Item = ();
}
