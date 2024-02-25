pub mod enums {
    use crate::teloxide::utils::command::BotCommands;

    #[derive(BotCommands, Clone)]
    #[command(rename = "lowercase", description = "List of all supported commands:")]
    pub enum Command {
        #[command(description = "Shows all available commands")]
        Help,
        #[command(description = "Shows how to format the query correctly")]
        FormatGuide,
    }

    #[derive(BotCommands, Clone)]
    #[command(rename = "lowercase", description = "Maintainer commands:")]
    pub enum MaintainerCommand {
        #[command(description = "How many users use this bot")]
        TotalUsers,
    }
}

/// This module is responsible for handling the commands.
pub mod executors {
    use super::enums::*;

    use std::error::Error;
    use std::sync::Arc;
    use teloxide::Bot;
    use teloxide::prelude::{AutoSend, Message, Requester};
    use crate::commands::FORMAT_GUIDE_MESSAGE;
    use crate::constants::MAINTAINER_ID;
    use crate::teloxide::utils::command::BotCommands;
    use crate::converter::ValidErrorKind;
    use crate::converter::error_messages;
    use crate::Journal;

    type HandlerResult = Result<(), Box<dyn Error + Sync + Send>>;

    pub async fn help(bot: AutoSend<Bot>, message: Message) -> HandlerResult {
        if message.from().unwrap().id == MAINTAINER_ID {
            bot.send_message(
                message.chat.id,
                &format!(
                    "{}\n\n{}",
                    Command::descriptions(),
                    MaintainerCommand::descriptions(),
                )
            ).await?;
        } else {
            bot.send_message(
                message.chat.id,
                Command::descriptions().to_string()
            ).await?;
        }
        Ok(())
    }

    pub async fn unknown_command(
        bot: AutoSend<Bot>,
        message: Message,
    ) -> HandlerResult {
        bot.send_message(message.chat.id, "Unknown command. Try /help").await?;
        Ok(())
    }

    pub async fn messages(
        bot: AutoSend<Bot>,
        message: Message,
        journal: Arc<Journal>,
    ) -> HandlerResult {
        bot.send_message(
            message.chat.id,
            format!("Total users: {}", journal.users())
        ).await?;
        Ok(())
    }

    pub async fn format_guide(bot: AutoSend<Bot>, message: Message) -> HandlerResult {
        bot.send_message(
            message.chat.id,
            FORMAT_GUIDE_MESSAGE,
        ).await?;

        Ok(())
    }

    pub async fn conversion_error(
        bot: AutoSend<Bot>,
        message: Message,
        error: ValidErrorKind,
        text: &str,
    ) -> HandlerResult {
        match error {
            ValidErrorKind::InvalidFormat => {
                bot.send_message(
                    message.chat.id, error_messages::INVALID_FORMAT_MESSAGE
                ).await?;
            }
            ValidErrorKind::InvalidMantissaLength => {
                bot.send_message(
                    message.chat.id, error_messages::INVALID_MANTISSA_LENGTH_MESSAGE
                ).await?;
            }
            ValidErrorKind::InvalidInitialBase => {
                bot.send_message(
                    message.chat.id, error_messages::INVALID_INITIAL_BASE_MESSAGE
                ).await?;
            }
            ValidErrorKind::InvalidInitialNumber => {
                bot.send_message(
                    message.chat.id, error_messages::INVALID_INITIAL_NUMBER_MESSAGE
                ).await?;
            }
            ValidErrorKind::InvalidConditionBase => {
                bot.send_message(
                    message.chat.id, error_messages::INVALID_CONDITION_BASE_MESSAGE
                ).await?;
            }
            ValidErrorKind::InvalidCondition => {
                bot.send_message(
                    message.chat.id, error_messages::INVALID_CONDITION_MESSAGE
                ).await?;
            }
            ValidErrorKind::UnknownError => {
                bot.send_message(MAINTAINER_ID, text).await?;
                bot.send_message(
                    message.chat.id, error_messages::UNKNOWN_ERROR_MESSAGE
                ).await?;
            }
        }

        Ok(())
    }

    pub async fn result(bot: AutoSend<Bot>, message: Message, result: &str) -> HandlerResult {
        bot.send_message(message.chat.id, result).await?;
        Ok(())
    }
}

const FORMAT_GUIDE_MESSAGE: &str =
    "Format for integer:\n\
    integer_numsys > desirednumsys\n\
    \n\
    Format for fractional:\n\
    fract_numsys > desirednumsys\n\
    or\n\
    fract_numsys > mantissalen_desirednumsys\n\
    \n\
    \n\
    Number system must be between 2 and 36 inclusively\n\
    Mantissa length must be between 0 and 50 inclusively";