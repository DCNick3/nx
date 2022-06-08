use crate::ipc::client;
use crate::ipc::sf;
use crate::ipc::sf::sm;
use crate::mem;
use crate::result::*;
use crate::service;

pub use crate::ipc::sf::psc::*;

pub struct PmModule {
    session: sf::Session,
}

impl sf::IObject for PmModule {
    ipc_sf_object_impl_default_command_metadata!();
}

impl IPmModule for PmModule {
    fn initialize(
        &mut self,
        id: ModuleId,
        dependencies: sf::InMapAliasBuffer<ModuleId>,
    ) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 0] (id, dependencies) => (event_handle: sf::CopyHandle))
    }

    fn get_request(&mut self) -> Result<(State, u32)> {
        ipc_client_send_request_command!([self.session.object_info; 1] () => (state: State, flags: u32))
    }

    fn acknowledge(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2] () => ())
    }

    fn finalize(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3] () => ())
    }

    fn acknowledge_ex(&mut self, state: State) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 4] (state) => ())
    }
}

impl client::IClientObject for PmModule {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }

    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }
}

pub struct PmService {
    session: sf::Session,
}

impl sf::IObject for PmService {
    ipc_sf_object_impl_default_command_metadata!();
}

impl IPmService for PmService {
    fn get_pm_module(&mut self) -> Result<mem::Shared<dyn IPmModule>> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => (pm_module: mem::Shared<PmModule>))
    }
}

impl client::IClientObject for PmService {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }

    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }
}

impl service::IService for PmService {
    fn get_name() -> sm::ServiceName {
        sm::ServiceName::new("psc:m")
    }

    fn as_domain() -> bool {
        true
    }

    fn post_initialize(&mut self) -> Result<()> {
        Ok(())
    }
}
