#![feature(nll)]
#![feature(const_fn)]
#![feature(maybe_uninit_ref)]

#[macro_use]
extern crate ctor;
extern crate mumble_sys;
use mumble_sys::types as m;

struct MumbleRPC {
    api: Option<mumble_sys::MumbleAPI>,
}

impl mumble_sys::traits::MumblePlugin for MumbleRPC {
    fn init(&mut self) -> m::Mumble_ErrorCode {
        println!("It's alive!");
        m::Mumble_ErrorCode::EC_OK
    }

    fn shutdown(&mut self) {
        println!("It's dead, Jim!");
    }

    fn set_api(&mut self, api: mumble_sys::MumbleAPI) {
        self.api = Some(api);
    }

    fn on_user_talking_state_changed(
        &mut self,
        conn: m::mumble_connection_t,
        user: m::mumble_userid_t,
        talking_state: m::talking_state_t,
    ) {
        let api = self.api.as_mut().unwrap();
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

    fn on_channel_added(&mut self, conn: m::mumble_connection_t, channel: m::mumble_channelid_t) {
        let api = self.api.as_mut().unwrap();
        if !api.is_connection_synchronized(conn) { return; }
        let channel_name = api.get_channel_name(conn, channel).unwrap();
        println!("Channel added {:?}:{:?} : {}", conn, channel, channel_name);
    }

    fn on_channel_entered(
        &mut self,
        conn: m::mumble_connection_t,
        user: m::mumble_userid_t,
        previous: Option<m::mumble_channelid_t>,
        current: Option<m::mumble_channelid_t>,
    ) {
        let api = self.api.as_mut().unwrap();
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
}

#[ctor]
fn set_registration_callback() {
    mumble_sys::set_registration_callback(Box::new(register_plugin))
}

fn register_plugin(token: mumble_sys::RegistrationToken) {
    let rpc = MumbleRPC { api: None };
    mumble_sys::register_plugin(
        "MumbleRPC",
        "Dessix",
        "A remote procedure call system for Mumble interop with other programs",
        m::Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        m::Version {
            major: 0,
            minor: 1,
            patch: 1,
        },
        Box::new(rpc),
        None,
        token,
    );
}
