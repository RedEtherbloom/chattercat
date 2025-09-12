use pipewire::context::Context as PipewireContext;
use pipewire::core::Core as PipewireCore;
use pipewire::main_loop::MainLoop;
use pipewire::registry::Registry as PipewireRegistry;

#[derive(Debug, Clone)]
struct Audio {
    main_loop: MainLoop,
    audio_context: PipewireContext,
    pipewire_core: PipewireCore,
    pipewire_registry: PipewireRegistry,
}

impl Audio {
    pub fn init() -> Result<Audio, Box<dyn std::error::Error>> {
        let main_loop = MainLoop::new(None)?;
        let audio_context = PipewireContext::new(&main_loop)?;
        let pipewire_core = audio_context.connect(None)?;
        let pipewire_registry = pipewire_core.get_registry()?;

        Audio {
            main_loop,
            audio_context,
            pipewire_core,
            pipewire_registry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
