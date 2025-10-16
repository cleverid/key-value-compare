use picodata_plugin::transport::rpc;
use picodata_plugin::{plugin::prelude::*, transport::rpc::RouteBuilder};
use serde::{Deserialize, Serialize};

use crate::models::Profile;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleResponse {
    pub rpc_hello_response: String,
}

pub fn register_example_rpc_handle(context: &PicoContext) {
    RouteBuilder::from_pico_context(context)
        .path("/greetings_rpc")
        .register(move |req, _ctx| {
            log::debug!("Received store request: {req:?}");

            let profile: Profile = rmp_serde::from_slice(req.as_bytes()).unwrap();

            log::warn!("Recieved \"{profile:?}\" as RPC input");

            let profile_id = profile.id;
            let response_to_return = ExampleResponse {
                rpc_hello_response: format!("Hello {profile_id}, long time no see."),
            };

            Ok(rpc::Response::encode_rmp(&response_to_return).unwrap())
        })
        .unwrap();
}
