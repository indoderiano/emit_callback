use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};
use crate::app::Msg as ParentMsg;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChildProps {
    pub callback: Callback<()>,
    pub callback2: Callback<String>,
    pub callback3: Callback<String>,
}

pub enum Msg {
    IncrementFromChild,
    SendDataFromChild2(String),
    SendDataFromChild3(String),
    TestMsg,
}

pub struct Child {
    link: ComponentLink<Self>,
    callback: Callback<()>,
    callback2: Callback<String>,
    callback3: Callback<String>,
}

impl Component for Child {
    type Message = Msg;
    type Properties = ChildProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        Self {
            link,
            callback: props.callback,
            callback2: props.callback2,
            callback3: props.callback3,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncrementFromChild => {
                // self.count += 1;
                self.callback.emit(());
            }
            Msg::SendDataFromChild2(data) => {
                // self.callback2.emit(Msg::GetData(data));
                // self.callback2.emit(ParentMsg::GetData(data));
                self.callback2.emit("text from child".to_string());
            }
            Msg::SendDataFromChild3(data) => {
                // self.callback2.emit(Msg::GetData(data));
                // self.callback2.emit(ParentMsg::GetData(data));
                self.callback3.emit("text from child".to_string());
            }
            Msg::TestMsg => {
                ConsoleService::info("test msg");
                // nothing
            }
        }
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "CHILD" }</h1>
                <button onclick=self.link.callback(|_| Msg::IncrementFromChild)>{ "+1 from child" }</button>
                <button onclick=self.link.callback(|_| Msg::SendDataFromChild2("text-from-child".to_string()))>{ "send string from child 2" }</button>
                <button onclick=self.link.callback(|_| Msg::SendDataFromChild3("text-from-child".to_string()))>{ "send string from child 3" }</button>
            </div>
        }
    }
}