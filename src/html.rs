use crate::db;
use crate::models::List;

use maud::{html, Markup, DOCTYPE};

pub fn make_card(card: db::Card) -> Markup {
    let id = format!("card-{}", card.id);

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
            (card.title)
        }
    }
}

/// A basic list with dynamic `list_title` and a list of cards.
pub fn make_list(list: List) -> Markup {
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
"
        {
            h2 class="list-title" { (list.title) }
            @for card in list.cards {
                (make_card(card))
            }
        }
    }
}

pub fn make_board(lists: Vec<List>) -> Markup {
    html! {
        div #board {
            @for list in lists {
                (make_list(list))
            }
        }
    }
}

pub fn base(board_title: String, lists: Vec<List>) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width,initial-scale=1.0";
                title { (format!("Board - {board_title}")) }
                link rel="stylesheet" type="text/css" href="/static/index.css";
                script src="/static/placement.js" {};
                script src="/static/DragDropTouch.js" {};
                script src="https://unpkg.com/hyperscript.org@0.9.12" {};
            }
            body {
                h1 { (board_title) }
                (make_board(lists))
            }
        }
    }
}
