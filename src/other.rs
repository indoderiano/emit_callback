use yew::prelude::*;

// Define a message struct to send from child to parent
pub struct Message(String);

// Define the child component's properties struct
#[derive(Properties, Clone)]
pub struct ChildProperties {
    pub on_send_message: Callback<Message>,
}

// Define a message enum for the child component
pub enum ChildMessage {
    UpdateMessage(String),
    SendMessage,
}

// Define a child component that takes a callback function as a prop
pub struct ChildComponent {
    on_send_message: Callback<Message>,
    message_input: String,
    link: ComponentLink<Self>,
}

impl Component for ChildComponent {
    type Message = ChildMessage;
    type Properties = ChildProperties;

    // Define the child component's properties
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            on_send_message: props.on_send_message,
            message_input: String::new(),
            link,
        }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        false
    }

    // Define the child component's view
    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    type="text"
                    value=self.message_input.clone()
                    // oninput=self.link.callback(|e: InputData| ChildMessage::UpdateMessage(e.value))
                />
                <button onclick=self.link.callback(|_| ChildMessage::SendMessage)>{"Send"}</button>
            </div>
        }
    }

    // Define the child component's update function
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Handle the update message to update the message input value
            ChildMessage::UpdateMessage(value) => {
                self.message_input = value;
                true
            }
            // Handle the send message to send the message input to the parent component
            ChildMessage::SendMessage => {
                self.on_send_message.emit(Message(self.message_input.clone()));
                self.message_input.clear();
                true
            }
        }
    }
}



// Define a parent component that receives the message from the child component
pub struct ParentComponent {
    message: Option<Message>,
    link: ComponentLink<Self>,
}

impl Component for ParentComponent {
    type Message = ();
    type Properties = ();

    // Define the parent component's create function
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { message: None, link, }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        false
    }

    // Define the parent component's view
    fn view(&self) -> Html {
        html! {
            <div>
                { "PARENT COMPONENT" }
                // <ChildComponent on_send_message=self.link.callback(|msg| Some(msg)) />
                {self.message.as_ref().map(|msg| html! { <p>{msg.0.clone()}</p> }).unwrap_or_default()}
            </div>
        }
    }

    // Define the parent component's update function
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Handle the message received from the child component
            () => {
                // self.message = self.link.current_state().children.iter().filter_map(|child| {
                //     if let Some(msg) = child.downcast_ref::<ChildComponent>().and_then(|c| c.message_input.as_ref()) {
                //         Some(Message(msg.clone()))
                //     } else {
                //         None
                //     }
                // }).next();
                true
            }
        }
    }
}