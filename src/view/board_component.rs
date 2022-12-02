use super::super::model::board::Board;
use super::case_component::{Props, CaseComponent};

use yew::Properties;
use yew::{prelude::{Component, Html, html}, props, AttrValue, Context};

pub struct BoardComponent {
    board : Board,
}

#[derive(PartialEq, Properties)]
pub struct RestartProps {
    pub restart : bool,
}

pub struct Msg {
    pub coord_x : usize,
    pub coord_y : usize
}

impl Component for BoardComponent {
    type Message = Msg;

    type Properties = RestartProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            board : Board::new(20, 20)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.board.player_plays(msg.coord_x, msg.coord_y);

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <table class="centered"> 
            {
                self.board
                    .get_cases()
                    .iter()
                    .enumerate()
                    .map( |(x, vec)| 
                        html! { 
                            <tr>
                            {
                                vec
                                .iter()
                                .enumerate()
                                .map(|(y, case)|
                                    {
                                        let pre_made_props = props! {
                                            Props {player : AttrValue::from(case.to_string())}
                                        };

                                        html! {
                                            <td class="case" onclick={link.callback(move |_| Msg{coord_x : x.clone(), coord_y: y.clone()})}>
                                                <CaseComponent ..pre_made_props />
                                            </td>
                                        }  
                                    }
                                      
                                ).collect::<Html>()
                            }
                            </tr> 
                        }
                    ).collect::<Html>()
            }
            </table>
        }

    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        self.board = Board::new(20, 20);
        return true;

        // if old_props.restart == false {
            
        // }
        
        // false
    }

}