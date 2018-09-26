extern crate dbus;
extern crate glib;
extern crate gtk;

mod mpris_player;
pub use mpris_player::MprisPlayer as MprisPlayer;

mod metadata;
pub use metadata::Metadata as Metadata;

mod status;
pub use status::PlaybackStatus as PlaybackStatus;
pub use status::LoopStatus as LoopStatus;

mod generated;
use generated::mediaplayer2::OrgMprisMediaPlayer2 as OrgMprisMediaPlayer2;
use generated::mediaplayer2_player::OrgMprisMediaPlayer2Player as OrgMprisMediaPlayer2Player;
