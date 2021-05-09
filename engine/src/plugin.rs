use crate::host_api::*;
use egui::CtxRef;
use libloading as lib;

pub struct Plugin {
    lib_path: String,
    libs: Vec<lib::Library>,
}

impl Plugin {
    pub fn new(lib_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let lib = unsafe { lib::Library::new(&lib_path)? };
        Ok(Self { libs: vec![lib], lib_path })
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let lib_path = self.lib_path.clone();
        std::mem::drop(self.libs.pop().unwrap());
        self.libs.push(unsafe { lib::Library::new(&lib_path)? });
        Ok(())
    }

    pub fn api(&mut self) -> Result<PluginApi<'_>, Box<dyn std::error::Error>> {
        unsafe {
            let init = self.libs[0].get(b"init")?;
            let update = self.libs[0].get(b"update")?;
            let dbg_update = self.libs[0].get(b"dbg_update")?;
            Ok(PluginApi {
                init,
                update,
                dbg_update,
            })
        }
    }
}

#[repr(C)]
pub struct GameState {
    _private: [u8; 0],
}

pub struct PluginApi<'lib> {
    pub init: lib::Symbol<'lib, fn(*mut dyn HostApi) -> *mut GameState>,
    pub update: lib::Symbol<'lib, fn(*mut GameState, &mut dyn HostApi, &Input) -> bool>,
    pub dbg_update: lib::Symbol<'lib, fn(*mut GameState, &mut dyn HostApi, &CtxRef) -> bool>,
}
