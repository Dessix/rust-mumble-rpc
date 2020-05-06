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
    fn init(&mut self) -> m::mumble_error_t {
        println!("It's alive!");
        m::ErrorCode_EC_OK
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
        talking_state: m::talking_state_t) {
        let api = self.api.as_mut().unwrap();
        let active_conn = api.get_active_server_connection();
        let user_in_conn = api.get_local_user_id(conn);
        println!("Hello from connection {} as user {}", conn, user);
    println!(
        "User {}{} in connection {} talking state is now {}",
        user,
        if user == user_in_conn && conn == active_conn { " (you)" } else { "" },
        conn,
        talking_state);
    }
}

#[ctor]
fn set_registration_callback() {
    mumble_sys::set_registration_callback(Box::new(register_plugin))
}

fn register_plugin(token: mumble_sys::RegistrationToken) {
    let rpc = MumbleRPC {
        api: None,
    };
    mumble_sys::register_plugin(
        "MumbleRPC",
        "Dessix",
        "A remote procedure call system for Mumble interop with other programs",
        m::Version { major: 1, minor: 0, patch: 0 },
        Box::new(rpc),
        None,
        token,
    );
}
