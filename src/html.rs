use crate::db;
use crate::models::List;

use maud::{html, Markup, DOCTYPE};

pub fn make_card(card: db::Card) -> Markup {
    let id = format!("card-{}", card.id);

    html! {
        li.card
            draggable="true"
            id=(id)
            hx-include="this"
            _="
on dragstart add .no-pointer-events to <.list>*/> when it is not me
    // add .no-pointer-events to the children of .list when it is not me
    then call event.dataTransfer.setData('text/plain', me.id)
on drop or dragend remove .no-pointer-events from <.list>*/>
    then remove .hovered from .list
    // remove .no-pointer-events from .no-pointer-events
"
        {
            input type="hidden" name="card-id" value=(card.id)

            span { (card.title) }
            button.remove hx-delete="/card" hx-target="#board" hx-swap="outerHTML" { "X" }
        }
    }
}

/// A basic list with dynamic `list_title` and a list of cards.
pub fn make_list(list: List) -> Markup {
    html! {
        ul.list id=(list.id()) _="
on dragover or dragenter halt the event
    remove .hovered from .list then add .hovered to me
on dragleave if event.target is me and event.fromElement.parentElement is not me
    remove .hovered from me
end
on drop remove .hovered from me
    get event.dataTransfer.getData('text/plain') then set card to #{it}
    if card exists then
        call determinePlacement(event, card) then set placement to it
        if placement exists then
            put placement.idx into #moved-new-position.value
            if placement.placeBefore then put card before placement.closestLi
            else put card after placement.closestLi end
        else
            put card at the end of me
            put '-1' into #moved-new-position.value
        end
        put strip_id(card.id) into #moved-card-id.value
        put strip_id(me.id) into #moved-to-list-id.value
        send cardmoved to #move-card
    end
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
        form.hidden id="move-card" hx-post="/card/move" hx-target="#board" hx-trigger="cardmoved" hx-swap="outerHTML" {
            input type="text" id="moved-card-id" name="card-id" value="" {}
            input type="text" id="moved-to-list-id" name="to-list-id" value="" {}
            input type="text" id="moved-new-position" name="new-position" value="" {}
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
                script type="text/hyperscript" {
                    "
                    def strip_id(s)
                        return s.split('-').pop()
                    "
                };
                script src="https://unpkg.com/hyperscript.org@0.9.12" {};
                script src="https://unpkg.com/htmx.org@1.9.9" {};
            }
            body {
                h1 { (board_title) }
                (make_board(lists))
            }
        }
    }
}
