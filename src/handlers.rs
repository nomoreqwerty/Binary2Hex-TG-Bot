use std::error::Error;
use std::sync::Arc;

use teloxide::prelude::{AutoSend, Message};

use crate::commands::enums::*;
use crate::commands::executors;
use crate::converter::{Converter, ValidErrorKind};
use crate::{debug, Journal};
use crate::teloxide::Bot;

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_command(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> HandlerResult {
    match command {
        Command::Help => {
            executors::help(bot, message).await?;
        },
        Command::FormatGuide => {
            executors::format_guide(bot, message).await?;
        }
    }

    Ok(())
}

pub async fn handle_maintainer_command(
    bot: AutoSend<Bot>,
    message: Message,
    command: MaintainerCommand,
    journal: Arc<Journal>
) -> HandlerResult {
    match command {
        MaintainerCommand::TotalUsers => {
            executors::messages(bot, message, journal).await?;
        }
    }

    Ok(())
}

pub async fn handle_message(
    bot: AutoSend<Bot>,
    message: Message,
    journal: Arc<Journal>,
) -> HandlerResult {
    if !journal.contains_user(message.from().unwrap().id) {
        journal.add_user(message.from().unwrap().id).unwrap();
    }

    if let Some(text) = message.text() {
        if let Some(text) = message.text() {
            if text == "/start" {
                executors::help(bot, message).await?;
                return Ok(());
            } else if text.starts_with("/") {
                executors::unknown_command(bot, message).await?;
                return Ok(());
            }
        }

        let converter = Converter::convert_from_message(text);

        if let Some(result) = converter.result {
            executors::result(bot, message, &result).await?;
        } else if let Some(error_kind) = converter.error_kind {
            let text = text.to_owned();
            if error_kind == ValidErrorKind::UnknownError {
                debug::print_debug_message(&format!("[ ERROR ] Unknown error from `{}`", text));
            }
            executors::conversion_error(bot, message, error_kind, &text).await?;
        }
    }

    Ok(())
}