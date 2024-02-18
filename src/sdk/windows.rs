pub struct WindowsSdk {
    modules: Vec<Module>,
    memory: Windows,
}

#[cfg(target_os = "windows")]
impl Sdk for WindowsSdk {
    fn new() -> Result<Self> {
        info!("initializing windows sdk");

        // System
        let mut system = System::new();
        system.refresh_all();

        let mut cs2_pid: usize = 0;

        // Get cs2 process ID
        let cs2_pids = system.processes_by_exact_name("cs2.exe");
        for cs2 in cs2_pids {
            cs2_pid = cs2.pid().into();
        }

        if cs2_pid == 0 {
            return Err(eyre::eyre!("failed to find cs2 process"));
        }

        debug!("found game process with pid: {}", cs2_pid);

        let memory: Windows = Memory::new(cs2_pid.into())?;
        let modules = vec![
            memory.get_module("client.dll")?,
            memory.get_module("engine2.dll")?,
        ];

        debug!("Modules found: {:?}", modules);

        Ok(Self { memory, modules })
    }

    fn get_memory(&self) -> &Windows {
        &self.memory
    }

    fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|module| module.name == name)
    }
}
