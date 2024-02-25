extern crate teloxide;
extern crate tokio;
extern crate single_instance;
extern crate rusqlite;

use std::sync::Arc;
use teloxide::{Bot, dptree};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::prelude::{Dispatcher, Message, RequesterExt};
use teloxide::types::Update;

use single_instance::SingleInstance;

mod digits;

mod converter;

mod journal;
use journal::Journal;

mod debug;

mod handlers;
use handlers::*;

mod constants;
use constants::TELEGRAM_BOT_TOKEN;

mod commands;

#[cfg(test)]
mod tests;

use commands::enums::*;
use crate::constants::MAINTAINER_ID;

#[tokio::main]
async fn main() {
    // Check if the bot is already running and exit if it is
    let instance = SingleInstance::new("Binary2HexBot").unwrap();
    if !instance.is_single() { return }

    debug::print_debug_message("Started Binary2Hex Bot");

    let bot = Bot::new(TELEGRAM_BOT_TOKEN).auto_send();

    let journal = Arc::new(Journal::new(r".\users.db").unwrap());
    journal.update();

    let handler = Update::filter_message()
        .branch(dptree::filter(|msg: Message| { msg.from().unwrap().id == MAINTAINER_ID })
            .branch(dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_command))

            .branch(dptree::entry()
                .filter_command::<MaintainerCommand>()
                .endpoint(handle_maintainer_command))

            .branch(dptree::entry()
                .endpoint(handle_message))
        )
        .branch(dptree::entry().filter_command::<Command>().endpoint(handle_command))
        .branch(dptree::entry().endpoint(handle_message));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![journal])
        .build()
        .dispatch()
        .await;
}
