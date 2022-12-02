use yew::{Html, html, Component, props};

use crate::view::board_component::{BoardComponent, RestartProps};

pub struct MainComponent {
    restart : bool,
}

pub struct MsgRestart{
    pub restart : bool
}

impl Component for MainComponent {
    type Message = MsgRestart;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            restart : false
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {

        let link = ctx.link();
        let pre_made_props = props! {RestartProps{restart : self.restart}};


        html!{
            <>
                <h1 class="text-center title"> {"Puissance 4"} </h1>
    
                <main>
                    <div class="percents-75">
                        <BoardComponent ..pre_made_props />
                    </div>

                    <button class="button-restart centered" onclick={link.callback(|_| MsgRestart{restart : true})}>
                        {"Recommencer"}
                    </button>  
                    
                </main>
            </>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.restart = msg.restart;

        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        if self.restart == true {
            self.restart = false;
            ctx.link().send_message(MsgRestart{restart : false});
        }
    }

}