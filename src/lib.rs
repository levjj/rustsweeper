mod model;
use model::{Cell, CellState, Pos};
pub use model::Model;
use rand::thread_rng;
use stdweb::traits::IEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

enum CellAction {
    Reveal,
    ToggleMark
}

pub enum Action {
    Reveal(Pos),
    ToggleMark(Pos),
    Restart
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
                    self.onreveal.as_ref().map_or(false, |s| { s.emit(()); true })
                } else {
                    false
                }
            }
            CellAction::ToggleMark => {
                if self.state != CellState::Revealed {
                    self.onmark.as_ref().map_or(false, |s| { s.emit(()); true })
                } else {
                    false
                }
            }
        }
    }
}

fn cell_to_class(cell: &CellModel) -> String {
    if cell.game_over {
        if cell.state == CellState::Marked {
            if cell.mine { String::from("correct") } else { String::from("incorrect") }
        } else {
            String::new()
        }
    } else {
        if cell.state == CellState::Revealed { String::new() } else { String::from("unknown") }
    }
}

fn cell_to_str(cell: &CellModel) -> String {
    if cell.state == CellState::Marked {
        String::from("⚑")
    } else if cell.state == CellState::Revealed || cell.game_over {
        if cell.mine {
            String::from("💣")
        } else if cell.neighbors == 0 {
            String::new()
        } else {
            cell.neighbors.to_string()
        }
    } else {
        String::new()
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

impl Component for Model {
    type Message = Action;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut model = Model::new(9, 9);
        model.place_mines(10, &mut thread_rng());
        model.calc_neighbors();
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Action::Reveal(pos) => {
                self.reveal(pos);
            },
            Action::ToggleMark(pos) => {
                self.toggle_marked(pos);
            },
            Action::Restart => {
                self.reset();
                self.place_mines(10, &mut thread_rng());
                self.calc_neighbors();
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
