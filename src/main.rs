#[cfg(debug_assertions)]
use std::{ptr::addr_of_mut, time::Instant};
#[cfg(debug_assertions)]
use iced::{window,Subscription};

use iced::{
    alignment::Horizontal, 
    widget::*, 
    Element, 
    Length, 
};

use inline_tweak::*;

fn run_app() {
    #[cfg(debug_assertions)]
    iced::application("Iced Inline Tweak", State::update, State::view)
        .subscription(State::subscription)
        .run()
        .unwrap()
    ;
    #[cfg(not(debug_assertions))]
    iced::application("Iced Inline Tweak", State::update, State::view)
        .run()
        .unwrap()
    ;
}

fn main() {
    run_app()
}

#[derive(Debug, Clone)]
enum Message {
    CounterUp,
    CounterDown,
    #[cfg(debug_assertions)]
    UpdateUI(bool),
}

#[derive(Debug, Default)]
struct State {
    value: i32,
    _upd_ui_: bool, // we require a change for request draw!
}

// Global state of the application
#[cfg(debug_assertions)]
static mut UPD_UI: bool = true;

#[cfg(debug_assertions)]
struct GlobalState{}

#[cfg(debug_assertions)]
impl GlobalState {
    // This function is used to toggle the state 
    // of the update UI subscription
    fn toggle_state(state_ui: bool) -> bool {
        let mut _upd_ui_ = addr_of_mut!(UPD_UI);
        unsafe{
            if *_upd_ui_ == state_ui {
                *_upd_ui_ = !*_upd_ui_;
            }
            *_upd_ui_
        }
    }

    // This function is used to get the state 
    // of the update UI subscription
    fn get_state() -> bool {
        let mut _upd_ui_ = addr_of_mut!(UPD_UI);
        unsafe {*_upd_ui_}
    }

}


impl State {

    // Raise the subscription to true or false 
    // depending on the state of the static variable
    // Every frame!
    #[cfg(debug_assertions)]
    fn subscription(_state: &State) -> Subscription<Message> {
        // change the state of the global variable to request a draw
        let _ = GlobalState::toggle_state(_state._upd_ui_);

        // Internal function for request the draw
        fn inner(_i: Instant) -> Message {
            Message::UpdateUI(GlobalState::get_state())
        }

        window::frames().map(inner)
    }

    #[tweak_fn]
    fn update(&mut self, message: Message) {
        match message {
            Message::CounterUp => self.value += 10,
            Message::CounterDown => self.value -= 1,
            #[cfg(debug_assertions)]
            Message::UpdateUI(b) => self._upd_ui_ = b, // change a value of the self struct to request a draw
        }
    }
    
    // This function is used to change the state of the UI
    #[tweak_fn]
    fn view(&self) -> Element<Message> {
        let title_text = 
        text("B!")
            .size(32.0)
            .color([1.0, 0.0, 0.0]);
        
        let up_button = button(text("Up"))
            .on_press(Message::CounterUp)
            .width(Length::FillPortion(3))
        ;
        let down_button = button(text("Down"))
            .on_press(Message::CounterDown)
            .width(Length::FillPortion(3))
        ;
        let counter_text = text(self.value.to_string())
            .width(Length::FillPortion(3))
            .size(40.0)
            .align_x(Horizontal::Center)
        ;
        let row = 
        row!(up_button, counter_text, down_button);
        
        iced::widget::column![
            title_text,
            row
        ].into()
    }
}