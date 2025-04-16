//! This module defines the [`CommandQueue`] structure, which facilitates communication in an
//! ECS (Entity-Component-System) context by providing a channel-based mechanism for sending
//! and receiving [`WorldCommand`]s.
//!
//! # Overview
//!
//! The [`CommandQueue`] acts as a command bus for the engine, enabling different parts
//! of the system to dispatch and consume commands in a thread-safe manner. It leverages
//! the `crossbeam_channel` crate to create unbounded channels for message passing.
//!
//! # Traits Implemented
//!
//! - [`WorldCommandReceiver`]: Allows the queue to act as a receiver for `WorldCommand`s, supporting
//!   non-blocking receive operations.
//! - [`WorldCommandSender`]: Provides an interface for sending `WorldCommand`s, ensuring safe
//!   communication between different parts of the system.

use std::fmt::Debug;
use crate::engine::{WorldCommand, WorldCommandReceiver, WorldCommandSender, WorldUserCommand, WorldUserCommandHandler};
use crossbeam_channel::TryRecvError;

/// A command queue for sending and receiving [`WorldCommand`]s within the engine.
///
/// This structure facilitates communication by providing a channel-based mechanism
/// for dispatching and handling commands in the ECS (Entity-Component-System) context.
pub struct CommandQueue<UserCommand> {
    /// The sending end of the command channel.
    sender: crossbeam_channel::Sender<WorldCommand<UserCommand>>,
    /// The receiving end of the command channel.
    receiver: crossbeam_channel::Receiver<WorldCommand<UserCommand>>,
}

impl<UserCommand> CommandQueue<UserCommand>  {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self { sender, receiver }
    }
}

impl<UserCommand> WorldUserCommand for CommandQueue<UserCommand>
where
    UserCommand: Send + Debug
{
    type UserCommand = UserCommand;
}

impl<UserCommand> WorldCommandReceiver for CommandQueue<UserCommand>
where
    UserCommand: Send + Debug
{
    type Error = TryRecvError;

    fn recv(&self) -> Result<Option<WorldCommand<Self::UserCommand>>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(cmd) => Ok(Some(cmd)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl<UserCommand> WorldCommandSender for CommandQueue<UserCommand>
where
    UserCommand: Send + Debug
{
    type Error = crossbeam_channel::SendError<WorldCommand<UserCommand>>;

    fn send(&self, command: WorldCommand<Self::UserCommand>) -> Result<(), Self::Error> {
        self.sender.send(command)
    }
}
