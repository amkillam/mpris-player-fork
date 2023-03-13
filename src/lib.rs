extern crate dbus;
extern crate dbus_tree as tree;
extern crate glib;

mod mpris_player;
pub use mpris_player::MprisPlayer;

mod metadata;
pub use metadata::Metadata;

mod status;
pub use status::LoopStatus;
pub use status::PlaybackStatus;

mod generated;
pub use generated::mediaplayer2::OrgMprisMediaPlayer2;
pub use generated::mediaplayer2_player::OrgMprisMediaPlayer2Player;
