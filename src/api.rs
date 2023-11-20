use askama_axum::Response;
use axum::{
    http::Request,
    middleware::{self, Next},
};
use serde::Deserialize;

// Example middleware that looks at response codes and sets some headers
// used by HTMX on the frontend. This isn't actually required but here
// for reference
async fn apply_hx_retarget<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut res = next.run(req).await;
    if res.status().as_u16() >= 400 {
        // res.headers_mut()
        //     .insert("HX-Retarget", "#any-errors".parse().unwrap());
        res.headers_mut()
            .insert("HX-ReSwap", "innerHTML".parse().unwrap());
    }
    res
}

pub fn api_routes() -> axum::Router<crate::db::DBPool> {
    axum::Router::new()
        .route("/todos", axum::routing::post(add_todo))
        .route("/todos/:id", axum::routing::delete(delete_todo))
        .route("/todos/:id/done", axum::routing::put(done_todo))
        .route("/todos/:id/text", axum::routing::put(change_text))
        .layer(middleware::from_fn(apply_hx_retarget))
}

async fn delete_todo(
    axum::extract::State(pool): axum::extract::State<crate::db::DBPool>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) {
    sqlx::query!("DELETE FROM todo WHERE id = $1", id)
        .execute(&pool)
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct AddTodoForm {
    text: String,
}

impl AddTodoForm {
    fn validate(&self) -> Result<(), (axum::http::StatusCode, String)> {
        const MIN_LEN: usize = 4;
        if self.text.len() < MIN_LEN {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                format!("Todo must be at least {MIN_LEN} characters"),
            ));
        }
        Ok(())
    }
}

async fn add_todo(
    axum::extract::State(pool): axum::extract::State<crate::db::DBPool>,
    axum::extract::Form(form): axum::extract::Form<AddTodoForm>,
) -> Result<crate::todo::TodoTemplate, (axum::http::StatusCode, String)> {
    form.validate()?;
    sqlx::query_as!(
        crate::TodoTemplate,
        "INSERT INTO todo(text) VALUES ($1) RETURNING *",
        form.text
    )
    .fetch_one(&pool)
    .await
    .map_err(crate::db::map_db_err)
}

async fn done_todo(
    axum::extract::State(pool): axum::extract::State<crate::db::DBPool>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<crate::todo::TodoTemplate, (axum::http::StatusCode, String)> {
    sqlx::query_as!(
        crate::TodoTemplate,
        r#"UPDATE todo SET done=not done WHERE id=$1 RETURNING id as "id!", text, done"#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(crate::db::map_db_err)
}

async fn change_text(
    axum::extract::State(pool): axum::extract::State<crate::db::DBPool>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    axum::extract::Form(form): axum::extract::Form<AddTodoForm>,
) -> Result<crate::todo::TodoTemplate, (axum::http::StatusCode, String)> {
    form.validate()?;
    sqlx::query_as!(
        crate::TodoTemplate,
        r#"UPDATE todo SET text=$2 WHERE id=$1 RETURNING id as "id!", text, done"#,
        id,
        form.text,
    )
    .fetch_one(&pool)
    .await
    .map_err(crate::db::map_db_err)
}
