use std::option::Option;

use rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use rbatis::sql::{Page, PageRequest};
use rbdc_pg::driver::PgDriver;

use crate::todo::todo::{get_next_id, select_page_data, Todo};

pub struct TodoService {
    rb: RBatis,
}

impl TodoService {
    pub fn new() -> TodoService {
        let rb = RBatis::new();
        rb.init(PgDriver {}, "postgres://postgres:password@localhost:5432/todo").unwrap();
        TodoService { rb }
    }

    pub async fn get_todo_list(&mut self) -> Result<Vec<Todo>, Error> {
        Todo::select_all(&mut self.rb).await
    }

    pub async fn get_todo_by_id(&mut self, id: u64) -> Result<Option<Todo>, Error> {
        Todo::select_by_id(&mut self.rb, id).await
    }

    pub async fn get_todo_page(&mut self, page: u64, limit: u64) -> Result<Page<Todo>, Error> {
        select_page_data(&mut self.rb, &PageRequest::new(page, limit)).await
    }

    pub async fn delete_todo_by_id(&mut self, id: u64) -> Result<ExecResult, Error> {
        Todo::delete_by_column(&mut self.rb, "id", id).await
    }

    pub async fn insert_todo(&mut self, todo: Todo) -> Result<Todo, Error> {
        let id = get_next_id(&mut self.rb).await;
        let new_todo = Todo {
            id: Option::from(id.unwrap()),
            owner: todo.owner,
            name: todo.name,
            description: todo.description,
            status: todo.status,
            created: Option::from(DateTime::now()),
            modified: Option::from(DateTime::now()),
        };
        let insert = Todo::insert(&mut self.rb, &new_todo).await;
        match insert {
            Ok(_) => { Result::Ok(new_todo) }
            Err(err) => { Result::Err(err) }
        }
    }

    pub async fn update_todo(&mut self, todo: Todo) -> Result<ExecResult, Error> {
        let update_todo = Todo {
            id: todo.id,
            owner: todo.owner,
            name: todo.name,
            description: todo.description,
            status: todo.status,
            created: todo.created,
            modified: Option::from(DateTime::now()),
        };
        Todo::update_by_column(&mut self.rb, &update_todo, "id").await
    }
}