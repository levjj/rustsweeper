use crate::model::CellState;
use stdweb::traits::IEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub enum Action {
    Reveal,
    ToggleMark,
}

#[derive(PartialEq, Clone, Default)]
pub struct Cell {
    pub mine: bool,
    pub neighbors: u8,
    pub state: CellState,
    pub game_over: bool,
    pub onreveal: Option<Callback<()>>,
    pub onmark: Option<Callback<()>>,
}

impl Component for Cell {
    type Message = Action;
    type Properties = Cell;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        props.clone()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.state == props.state && self.game_over == props.game_over {
            return false;
        }
        self.state = props.state;
        self.mine = props.mine;
        self.neighbors = props.neighbors;
        self.game_over = props.game_over;
        self.onreveal = props.onreveal;
        self.onmark = props.onmark;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Action::Reveal => {
                if self.state == CellState::Unmarked {
                    self.onreveal.as_ref().map_or(false, |s| {
                        s.emit(());
                        true
                    })
                } else {
                    false
                }
            }
            Action::ToggleMark => {
                if self.state != CellState::Revealed {
                    self.onmark.as_ref().map_or(false, |s| {
                        s.emit(());
                        true
                    })
                } else {
                    false
                }
            }
        }
    }
}

fn cell_to_class(cell: &Cell) -> String {
    match cell {
        Cell { game_over: true, state: CellState::Marked, mine: true, .. } => String::from("correct"),
        Cell { game_over: true, state: CellState::Marked, mine: false, .. } => String::from("incorrect"),
        Cell { game_over: false, state, .. } if *state != CellState::Revealed => String::from("unknown"),
        _ => String::new()
    }
}

fn cell_to_str(cell: &Cell) -> String {
    let visible = cell.game_over || cell.state == CellState::Revealed;
    match cell {
        Cell { state: CellState::Marked, .. } => String::from("⚑"),
        Cell { mine: true, .. } if visible => String::from("💣"),
        Cell { mine: false, neighbors: 1..=8, .. } if visible => cell.neighbors.to_string(),
        _ => String::new()
    }
}

impl Renderable<Cell> for Cell {
    fn view(&self) -> Html<Self> {
        html! {
            <td>
                <button
                class=cell_to_class(&self)
                disabled=self.game_over
                onclick=|_| Action::Reveal
                oncontextmenu=|e| { e.prevent_default(); Action::ToggleMark }>
                { cell_to_str(&self) }
                </button>
            </td>
        }
    }
}
