use crate::model::{Cell, CellState, Model, Pos};
use stdweb::traits::IEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

enum CellAction {
    Reveal,
    ToggleMark,
}

pub enum Action {
    Reveal(Pos),
    ToggleMark(Pos),
    Restart,
}

#[derive(PartialEq, Clone, Default)]
struct CellModel {
    mine: bool,
    neighbors: u8,
    state: CellState,
    game_over: bool,
    onreveal: Option<Callback<()>>,
    onmark: Option<Callback<()>>,
}

impl Component for CellModel {
    type Message = CellAction;
    type Properties = CellModel;

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
            CellAction::Reveal => {
                if self.state == CellState::Unmarked {
                    self.onreveal.as_ref().map_or(false, |s| {
                        s.emit(());
                        true
                    })
                } else {
                    false
                }
            }
            CellAction::ToggleMark => {
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

fn cell_to_class(cell: &CellModel) -> String {
    match cell {
        CellModel { game_over: true, state: CellState::Marked, mine: true, .. } => String::from("correct"),
        CellModel { game_over: true, state: CellState::Marked, mine: false, .. } => String::from("incorrect"),
        CellModel { game_over: false, state, .. } if *state != CellState::Revealed => String::from("unknown"),
        _ => String::new()
    }
}

fn cell_to_str(cell: &CellModel) -> String {
    let visible = cell.game_over || cell.state == CellState::Revealed;
    match cell {
        CellModel { state: CellState::Marked, .. } => String::from("⚑"),
        CellModel { mine: true, .. } if visible => String::from("💣"),
        CellModel { mine: false, neighbors: 1..=8, .. } if visible => cell.neighbors.to_string(),
        _ => String::new()
    }
}

impl Renderable<CellModel> for CellModel {
    fn view(&self) -> Html<Self> {
        html! {
            <td>
                <button
                class=cell_to_class(&self)
                disabled=self.game_over
                onclick=|_| CellAction::Reveal
                oncontextmenu=|e| { e.prevent_default(); CellAction::ToggleMark }>
                { cell_to_str(&self) }
                </button>
            </td>
        }
    }
}

const NUMBER_OF_MINES: u8 = 10;

impl Component for Model {
    type Message = Action;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut model = Model::new(9, 9);
        model.prepare_mines(NUMBER_OF_MINES);
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Action::Reveal(pos) => {
                self.reveal(pos);
            }
            Action::ToggleMark(pos) => {
                self.toggle_marked(pos);
            }
            Action::Restart => {
                self.reset();
                self.prepare_mines(NUMBER_OF_MINES);
            }
        }
        true
    }
}

fn view_cell(x: usize, y: usize, cell: &Cell, game_over: bool) -> Html<Model> {
    html! {
        <CellModel
          state=cell.state.clone()
          mine=cell.mine
          neighbors=cell.neighbors
          game_over=game_over
          onreveal=move |_| Action::Reveal((x as u8, y as u8))
          onmark=move |_| Action::ToggleMark((x as u8, y as u8)) />
    }
}

fn view_row(y: usize, row: &Vec<Cell>, game_over: bool) -> Html<Model> {
    html! {
        <tr>
            { for row.iter().enumerate().map(|(x, cell)| view_cell(x, y, cell, game_over))  }
        </tr>
    }
}

fn view_grid(model: &Model) -> Html<Model> {
    let game_over = model.game_over();
    let grid = model.to_grid();
    html! {
        <table>
            { for grid.iter().enumerate().map(|(y, row)| view_row(y, row, game_over) )  }
        </table>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <main>
                <h1>{ "Rustsweeper" }</h1>
                <nav>
                    <button onclick=|_| Action::Restart>{ "Restart" }</button>
                    <p>{ self.message() }</p>
                    <div style="clear:both"></div>
                </nav>
                { view_grid(self) }
            </main>
        }
    }
}
