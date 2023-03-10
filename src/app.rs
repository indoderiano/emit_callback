use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};
use crate::child::Child;

pub enum Msg {
    Increment,
    Decrement,
    GetData(String),
}

pub struct App {
    link: ComponentLink<Self>,
    count: i16
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        Self {
            link,
            count: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                ConsoleService::info("incrementing");
                self.count += 1;
            }
            Msg::Decrement => {
                ConsoleService::info("decrementing");
                self.count -= 1;
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("data in parent is {:?}", data));
            }
        }
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // let cb2 = self.link.callback(|data: String| Msg::GetData(data));
        let link = self.link.clone();
        html! {
            <div>
                <h1>{ "TESTING EMIT CALLBACK" }</h1>
                <h1>{ self.count }</h1>
                <button onclick=self.link.callback(|_| Msg::Increment)>{ "+1" }</button>
                // <Child callback=Callback::from(|_| {
                //     ConsoleService::info("decrementingasdfasdf");
                //     // self.count = 9;
                //     })
                // />
                <Child
                    callback=self.link.callback(|_| Msg::Increment)
                    // callback2=self.link.callback(|_| Msg::GetData("asdf".to_string()))
                    callback2=self.link.callback(Msg::GetData)
                    callback3=Callback::from(move |data: String| {
                        ConsoleService::info(&data.clone());
                        link.clone().send_message(Msg::GetData(data));
                    })
                    // callback2=cb2
                />
            </div>
        }
    }
}