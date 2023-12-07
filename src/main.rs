use actix_files::Files;
use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::{html, Markup, DOCTYPE};
use std::io;
use std::sync::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

fn card(data: &str) -> Markup {
    let id = {
        let mut counter = COUNTER.lock().unwrap(); // Access and modify the counter
        let id = format!("card-{}", *counter);
        *counter += 1;
        id
    };

    html! {
        li.card
            draggable="true"
            id=(id)
            _="
on dragstart add .no-pointer-events to <.list>*/> when it is not me
    // add .no-pointer-events to the children of .list when it is not me
    then call event.dataTransfer.setData('text/plain', me.id)
on drop or dragend remove .no-pointer-events from <.list>*/>
    then remove .hovered from .list
    // remove .no-pointer-events from .no-pointer-events
"
        {
            (data)
        }
    }
}

/// A basic list with dynamic `list_title` and a list of cards.
fn list(list_title: &str, cards: Vec<&str>) -> Markup {
    html! {
        ul.list _="
on dragover or dragenter halt the event then add .hovered to me
on dragleave if event.target is me and event.fromElement.parentElement is not me
    remove .hovered from me
end
on drop remove .hovered from me
on drop get event.dataTransfer.getData('text/plain')
    then set card to #{it}
    then call determinePlacement(event) then set placement to it
    if placement exists then
        if placement.placeBefore then put card before placement.closestLi
        else put card after placement.closestLi end
    else put card at the end of me end
" {
            h2 class="list-title" { (list_title) }
            @for card_data in cards {
                (card(card_data))
            }
        }
    }
}

/// The final Markup, including columns and cards.
pub fn board(board_title: &str, columns: Vec<(String, Vec<&str>)>) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { (board_title) }
                link rel="stylesheet" type="text/css" href="/static/index.css";
                script src="/static/placement.js" {};
                script src="/static/DragDropTouch.js" {};
                script src="https://unpkg.com/hyperscript.org@0.9.12" {};
            }
            body {
                h1 { (board_title) }

                div #board {
                    @for (list_title, cards) in columns {
                        (list(&list_title, cards))
                    }
                }
            }
        }
    }
}

#[get("/")]
async fn index() -> AwResult<Markup> {
    Ok(board(
        "My board",
        vec![
            (String::from("To do"), vec!["Fix #3", "Refactor everything"]),
            (String::from("Doing"), vec!["Fix #2"]),
            (String::from("Done"), vec!["Start project", "Fix #1"]),
        ],
    ))
}

#[get("/hello")]
async fn hello() -> AwResult<Markup> {
    Ok(html! {
        html {
            body {
                h1 { "Hello World!" }
            }
        }
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
