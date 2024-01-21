use color_eyre::Result;
use log::info;

mod memory;
mod sdk;

/*
   game_entity_system = client.dll + 0x158D0B8

   __inline auto get_base_player_controller_from_idx(DWORD64 game_entity_system, int idx) -> DWORD64 {
       if ((std::uint32_t)idx >= 0x7FFF)
           return{};

       auto controller_chunk = memory::proc::cs2.read<DWORD64>(game_entity_system + 8i64 * (idx >> 9) + 16);
       if (!controller_chunk)
           return {};

       auto controller = controller_chunk + 120i64 * (idx & 0x1FF);
       if (!controller)
           return {};

       auto controller_ent_idx = memory::proc::cs2.read<std::uint32_t>(controller + 16) & 0x7FFF;
       if (controller_ent_idx != idx)
           return {};

       return memory::proc::cs2.read<DWORD64>(controller);
   }

   __inline auto get_base_entity_from_controller(DWORD64 controller) -> DWORD64 {
       auto entity_list = memory::mod::client.read<DWORD64>(0x1519FD8);
       if (!entity_list)
           return {};

       auto pawn_handle = memory::proc::cs2.read<std::uint32_t>(controller + 0x5D4);
       if (!pawn_handle || pawn_handle > 0xFFFFFFFD)
           return {};

       auto identities_chunk = memory::proc::cs2.read<DWORD64>(entity_list + 8 * ((DWORD64)(pawn_handle & 0x7FFF) >> 9));
       if (!identities_chunk)
           return {};

       auto identity = identities_chunk + 120i64 * (pawn_handle & 0x1FF);
       if (!identity)
           return {};

       auto identity_ent_handle = memory::proc::cs2.read<std::uint32_t>(identity + 0x10);
       if (identity_ent_handle != pawn_handle)
           return {};

       return memory::proc::cs2.read<DWORD64>(identity);
   }
*/
fn populate_entity_list() {
    let cs2 = memory::from_name("cs2.exe").unwrap();

    let modules_to_get = ["engine2.dll", "client.dll"];
    for module in modules_to_get {
        let module_handle = cs2.get_module(module).unwrap();
        if module == "client.dll" {
            let entity_list: usize = cs2
                .read::<usize>(module_handle.base + *offset_entity_list)
                .unwrap();
            info!("{} base: {:#x}", module, module_handle.base);

            for player in 0..32 {
                // let entity_list_entry = entity_list[(player & 0x7FFF) >> 9];
            }
        }
    }
}

fn main() -> Result<()> {
    // Install color_eyre as the global error handler
    color_eyre::install().unwrap();

    // Setup tracing subscriber
    tracing_subscriber::fmt::init();

    // Populate cached entity list
    populate_entity_list();

    Ok(())
}
