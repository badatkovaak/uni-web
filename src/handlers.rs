// use axum::http;

pub async fn echo() -> axum::Json<&'static str>{
    return axum::Json("Hi There!");
}
