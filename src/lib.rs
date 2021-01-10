#![feature(nll)]
#![feature(const_fn)]
#![feature(maybe_uninit_ref)]

extern crate mumble_sys;
use mumble_sys::types as m;
use mumble_sys::types::UserIdT;

struct MumbleRPC {
    api: mumble_sys::MumbleAPI,
}

impl mumble_sys::traits::MumblePlugin for MumbleRPC {
    fn shutdown(&self) {
        println!("It's dead, Jim!");
    }

    fn on_channel_entered(
        &mut self,
        conn: m::ConnectionT,
        user: m::UserIdT,
        previous: Option<m::ChannelIdT>,
        current: Option<m::ChannelIdT>,
    ) {
        let api = &mut self.api;
        if !api.is_connection_synchronized(conn) { return; }
        let previous_channel_name = previous
            .map(|c| api.get_channel_name(conn, c).unwrap())
            .unwrap_or(String::from("<None>"));
        let channel_name = current
            .map(|c| api.get_channel_name(conn, c).unwrap())
            .unwrap_or(String::from("<None>"));
        let user_name = api
            .get_user_name(conn, user)
            .unwrap_or(String::from("<Unavailable>"));
        let user_hash = api
            .get_user_hash(conn, user)
            .unwrap_or(String::from("<Unavailable>"));
        let server_hash = api
            .get_server_hash(conn)
            .unwrap_or(String::from("<Unavailable>"));
        println!(
            "User {} ({}) entered {} from {} on server {}",
            user_name, user_hash, channel_name, previous_channel_name, server_hash
        );
    }

    fn on_user_talking_state_changed(
        &mut self,
        conn: m::ConnectionT,
        user: m::UserIdT,
        talking_state: m::TalkingStateT,
    ) {
        let api = &mut self.api;
        if !api.is_connection_synchronized(conn) { return; }
        let local_user = api.get_local_user_id(conn).unwrap();
        println!(
            "User #{:?} {}({}){} in connection {:?} talking state is now {:?}",
            user,
            api.get_user_name(conn, user).unwrap(),
            api.get_user_hash(conn, user).unwrap(),
            if user == local_user { " (you)" } else { "" },
            conn,
            talking_state
        );
    }

    fn on_audio_source_fetched(
        &mut self,
        pcm: &mut [f32],
        sample_count: u32,
        channel_count: u16,
        sample_rate: u32,
        is_speech: bool,
        user_id: Option<UserIdT>,
    ) -> bool {
        if !is_speech {
            return false;
        }
        let user_id = user_id.unwrap();
        println!(
            "Received speech with {} samples ({}hz) in {} channels from user {:?}",
            sample_count,
            sample_rate,
            channel_count,
            user_id
        );
        false
    }

    fn on_channel_added(&mut self, conn: m::ConnectionT, channel: m::ChannelIdT) {
        let api = &mut self.api;
        if !api.is_connection_synchronized(conn) { return; }
        let channel_name = api.get_channel_name(conn, channel).unwrap();
        println!("Channel added {:?}:{:?} : {}", conn, channel, channel_name);
    }
}

impl mumble_sys::traits::MumblePluginDescriptor for MumbleRPC {
    fn name() -> &'static str {
        println!("Name requested");
        "MumbleRPC"
    }

    fn author() -> &'static str {
        println!("Author requested");
        "Dessix"
    }

    fn description() -> &'static str {
        println!("Description requested");
        "A remote procedure call system for Mumble interop with other programs"
    }

    fn version() -> m::Version {
        println!("Version requested");
        m::Version { major: 0, minor: 0, patch: 1 }
    }

    fn api_version() -> m::Version {
        println!("APIVersion requested");
        m::Version { major: 1, minor: 0, patch: 0 }
    }

    fn init(id: m::PluginId, api: m::MumbleAPI) -> Result<Self, m::ErrorT> {
        println!("It's alive!");
        Ok(MumbleRPC { api: mumble_sys::MumbleAPI::new(id, api) })
    }

}

mumble_sys::register_mumble_plugin!(MumbleRPC);
