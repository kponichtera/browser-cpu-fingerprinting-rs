use std::rc::Rc;

use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::clock::ClockWorker;

use super::ClockMessage;

pub struct ClockBridge {
    bridge: Box<dyn Bridge<ClockWorker>>,
    counter: i64,
}

impl Component for ClockBridge {
    type Message = ClockMessage;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let callback = move |count| {
            link.send_message(ClockMessage::Count(count));
        };

        ClockBridge {
            bridge: ClockWorker::bridge(Rc::new(callback)),
            counter: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ClockMessage::Start => {
                self.bridge.send(());
            }
            ClockMessage::Count(count) => self.counter = count,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let run_worker = {
            let link = ctx.link().clone();
            Callback::from(move |_| {
                link.send_message(ClockMessage::Start);
            })
        };

        html! {
            <div>
                <button onclick={run_worker}>{"Run worker"}</button>
                {&self.counter}
            </div>
        }
    }
}
