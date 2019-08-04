mod model;
pub use model::Model;
use model::{CellView, Pos};
use rand::thread_rng;
use stdweb::traits::IEvent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub enum Action {
    Reveal(Pos),
    ToggleMark(Pos),
    Restart,
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
            Action::Reveal(pos) => self.reveal(pos),
            Action::ToggleMark(pos) => self.toggle_marked(pos),
            Action::Restart => {
                self.reset();
                self.place_mines(10, &mut thread_rng());
                self.calc_neighbors();
                true
            }
        }
    }
}

fn cell_to_class(cell: &CellView) -> String {
    match &cell {
        CellView::Marked => String::from("unknown"),
        CellView::Unknown => String::from("unknown"),
        _ => String::from("revealed"),
    }
}

fn cell_to_str(cell: &CellView) -> String {
    match &cell {
        CellView::Empty(0) => String::new(),
        CellView::Empty(n) => n.to_string(),
        CellView::Mine => String::from("💣"),
        CellView::Marked => String::from("⚑"),
        CellView::Unknown => String::new(),
    }
}

fn view_cell(x: usize, y: usize, cell: &CellView, disabled: bool) -> Html<Model> {
    html! {
        <td>
            <button
              class=cell_to_class(cell)
              disabled=disabled
              onclick=|e| Action::Reveal((x as u8, y as u8))
              oncontextmenu=|e| { e.prevent_default(); Action::ToggleMark((x as u8, y as u8)) }>
              { cell_to_str(cell) }
            </button>
        </td>
    }
}

fn view_row(y: usize, row: &Vec<CellView>, disabled: bool) -> Html<Model> {
    html! {
        <tr>
            { for row.iter().enumerate().map(|(x, cell)| view_cell(x, y, cell, disabled))  }
        </tr>
    }
}

fn view_grid(model: &Model) -> Html<Model> {
    let disabled = model.game_over();
    let grid = model.to_grid();
    html! {
        <table>
            { for grid.iter().enumerate().map(|(y, row)| view_row(y, row, disabled) )  }
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
