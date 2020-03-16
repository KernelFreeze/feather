//! Systems which send packets based on events.
//!
//! There are four types of broadcasters:
//! * Those which broadcast packets to all online clients through `Game::broadcast_global()`.
//! * Those which broadcast packets to all clients who can see a given entity through `Game::broadcast_entity_update()`.
//! * Those which send additional packets, such as equipment, etc. after entity spawning
//! packets have been sent. This is done through `EntitySendEvent`.
//! * Those which just send a packet to a single player.

mod animation;
mod block;
mod chat;
pub mod entity_creation;
pub mod entity_deletion;
mod inventory;
mod item_collect;
pub mod keepalive;
mod metadata;
pub mod movement;
