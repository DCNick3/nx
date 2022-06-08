use crate::ipc::client;
use crate::ipc::sf::{self, sm};
use crate::result::*;
use crate::service;

pub use crate::ipc::sf::psm::*;

pub struct PsmServer {
    session: sf::Session,
}

impl sf::IObject for PsmServer {
    ipc_sf_object_impl_default_command_metadata!();
}

impl IPsmServer for PsmServer {
    fn get_battery_charge_percentage(&mut self) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => (charge: u32))
    }
}

impl client::IClientObject for PsmServer {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }

    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }
}

impl service::IService for PsmServer {
    fn get_name() -> sm::ServiceName {
        sm::ServiceName::new("psm")
    }

    fn as_domain() -> bool {
        false
    }

    fn post_initialize(&mut self) -> Result<()> {
        Ok(())
    }
}
