use pipewire::context::Context as PipewireContext;
use pipewire::core::Core as PipewireCore;
use pipewire::main_loop::MainLoop;
use pipewire::registry::Registry as PipewireRegistry;
use pipewire::spa as spa;
use pipewire::stream::Stream as PipewireStream;

#[derive(Debug, Clone)]
struct Audio {
    main_loop: MainLoop,
    audio_context: PipewireContext,
    pipewire_core: PipewireCore,
    pipewire_registry: PipewireRegistry,
    audio_sink: Option<PipewireStream>,
    audio_listener: Option<todo!("Till type")>,
}

// TODO: Turn into a trait
impl Audio {
    pub fn init() -> Result<Audio, Box<dyn std::error::Error>> {
        let main_loop = MainLoop::new(None)?;
        let audio_context = PipewireContext::new(&main_loop)?;
        let pipewire_core = audio_context.connect(None)?;
        let pipewire_registry = pipewire_core.get_registry()?;

        Ok(Audio {
            main_loop,
            audio_context,
            pipewire_core,
            pipewire_registry,
            audio_sink: None,
        })
    }

    pub fn create_sink(&mut self, name: &str) -> Result<()> {
        let sink_props = pipewire::properties::properties! {
            // TODO: Understand diff for sink/source for capturing
            *pipewire::keys::MEDIA_CLASS => "Audio/Sink",
            // Are MEDIA_ROLE or MEDIA_TYPE required?
            *pipewire::keys::MEDIA_TYPE => "Audio",
        };
        // TODO: Auto-connect to default?

        audio_sink = PipewireStream::new(self.core, name, sink_props)?;
        // TODO: Auto-connect to default
        // What stream fplags to set? Autoconnect? Mapping buffers?
        let _listener = audio_sink
            .add_local_listener()
            .param_changed(|_, _, param_id, param| {
                // TODO: Lookup this case in the docs. The example handles this the same way but I have
                // no good idea why it does it like this.
                if param.is_none() {
                    return;
                }
                // Only handle the param type for now
                if param_id != spa::param::ParamType::Format.as_raw() {
                    return;
                }

                let (media_type, media_subtype) = spa::param::format_utils::parse_format(param).or_else
            });
        // audio_sink.connect(spa::utils::Direction::Input, None, )

        // self.audio_sink = Ok(audio_sink);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
