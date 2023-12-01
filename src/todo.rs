#[derive(askama::Template)]
#[template(path = "components/todo.html")]
pub struct TodoTemplate {
    pub id: i64,
    pub text: String,
    pub done: bool,
}

#[derive(askama::Template)]
#[template(path = "components/todo_stat.html")]
pub struct TodoStatTemplate {
    pub value: i32,
}

#[derive(askama::Template)]
#[template(path = "components/todo_piechart.html")]
pub struct TodoPiechartTemplate {
    pub sum_open: i32,
    pub sum_closed: i32,
}
