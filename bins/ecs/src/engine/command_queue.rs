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

use crate::engine::{
    NoUserCommand, WorldCommand, WorldCommandReceiver, WorldCommandSender, WorldUserCommand,
};
use crossbeam_channel::TryRecvError;

type Command = WorldCommand<NoUserCommand>;

/// A command queue for sending and receiving [`WorldCommand`]s within the engine.
///
/// This structure facilitates communication by providing a channel-based mechanism
/// for dispatching and handling commands in the ECS (Entity-Component-System) context.
pub struct CommandQueue {
    /// The sending end of the command channel.
    sender: crossbeam_channel::Sender<Command>,
    /// The receiving end of the command channel.
    receiver: crossbeam_channel::Receiver<Command>,
}

impl CommandQueue {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self { sender, receiver }
    }
}

impl WorldUserCommand for CommandQueue {
    type UserCommand = NoUserCommand;
}

impl WorldCommandReceiver for CommandQueue {
    type Error = TryRecvError;

    fn recv(&self) -> Result<Option<Command>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(cmd) => Ok(Some(cmd)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl WorldCommandSender for CommandQueue {
    type Error = crossbeam_channel::SendError<Command>;

    fn send(&self, command: Command) -> Result<(), Self::Error> {
        self.sender.send(command)
    }
}
