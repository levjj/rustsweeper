use crate::model::{Cell, Field, Pos};
use crate::ui::cell::Cell as CellComponent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

const NUMBER_OF_MINES: u8 = 10;

pub enum Action {
    Reveal(Pos),
    ToggleMark(Pos),
    Restart,
}

impl Component for Field {
    type Message = Action;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut field = Field::new(9, 9);
        field.prepare_mines(NUMBER_OF_MINES);
        field
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

fn view_cell(x: usize, y: usize, cell: &Cell, game_over: bool) -> Html<Field> {
    html! {
        <CellComponent
          state=cell.state.clone()
          mine=cell.mine
          neighbors=cell.neighbors
          game_over=game_over
          onreveal=move |_| Action::Reveal((x as u8, y as u8))
          onmark=move |_| Action::ToggleMark((x as u8, y as u8)) />
    }
}

fn view_row(y: usize, row: &[Cell], game_over: bool) -> Html<Field> {
    html! {
        <tr>
            { for row.iter().enumerate().map(|(x, cell)| view_cell(x, y, cell, game_over))  }
        </tr>
    }
}

fn view_grid(field: &Field) -> Html<Field> {
    let game_over = field.game_over();
    let grid = field.to_field();
    html! {
        <table>
            { for grid.iter().enumerate().map(|(y, row)| view_row(y, row, game_over) )  }
        </table>
    }
}

impl Renderable<Field> for Field {
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
