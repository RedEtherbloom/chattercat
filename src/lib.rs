use pipewire::context::Context as PipewireContext;
use pipewire::core::Core as PipewireCore;
use pipewire::main_loop::MainLoop;
use pipewire::registry::Registry as PipewireRegistry;
use pipewire::spa;
use pipewire::spa::param::audio::AudioInfoRaw;
use pipewire::spa::param::format::{MediaSubtype, MediaType};
use pipewire::stream::Stream as PipewireStream;
use spa::param::ParamType;

#[derive(Debug)]
struct Audio {
    main_loop: MainLoop,
    audio_context: PipewireContext,
    pipewire_core: PipewireCore,
    pipewire_registry: PipewireRegistry,
    audio_sink: Option<PipewireStream>,
    // audio_listener: Option<todo!("Till type")>,
    info: Option<StreamInfo>,
}

// TODO: Make Send+Sync
#[derive(Debug, Clone)]
struct StreamInfo {
    raw_info: Option<AudioInfoRaw>,
}

// TODO: Turn into a builder pattern
// TODO: Make audio-stack agnostic
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
            // audio_listener: None,
            info: None,
        })
    }

    pub fn create_sink(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sink_props = pipewire::properties::properties! {
            // TODO: Understand diff for sink/source for capturing
            *pipewire::keys::MEDIA_CLASS => "Audio/Sink",
            // Are MEDIA_ROLE or MEDIA_TYPE required?
            *pipewire::keys::MEDIA_TYPE => "Audio",
            // TODO: Can a pref for e.g. accepted formats be told here as well?
        };
        // TODO: Auto-connect to default?

        let audio_sink = PipewireStream::new(&self.pipewire_core, name, sink_props)?;
        // TODO: Consider option instead
        // TODO: Auto-connect to default
        // What stream fplags to set? Autoconnect? Mapping buffers?
        let _listener = audio_sink
            .add_local_listener_with_user_data(StreamInfo { raw_info: None })
            // TODO: Move into it's own function
            .param_changed(|_, user_data, param_id, param| {
                // TODO: Lookup this case in the docs. The example handles this the same way but I have
                // no good idea why it does it like this.
                let Some(param) = param else {
                    return;
                };

                match ParamType::from_raw(param_id) {
                    ParamType::Format => {
                        // Invalidate prev
                        user_data.raw_info = None;

                        // TODO:: Format change reaction
                        let (media_type, media_subtype) =
                            match spa::param::format_utils::parse_format(param) {
                                Ok((t, st)) => (t, st),
                                Err(e) => {
                                    println!("Could not parse Format format: {:#?}", e);
                                    return;
                                }
                            };

                        if media_type != MediaType::Audio {
                            println!("Rejecting non-audio media type: {:#?}", media_type);
                            return;
                        }
                        if media_subtype != MediaSubtype::Raw {
                            println!("Rejecting non-raw submedia type: {:#?}", media_subtype);
                            return;
                        }

                        println!(
                            "Proceeding with accepted media typing: {:#?}/{:#?}",
                            media_type, media_subtype
                        );

                        let mut audio_info = AudioInfoRaw::new();
                        match &mut audio_info.parse(param) {
                            Ok(info) => println!("Successfully parsed info: {:#?}", info),
                            Err(e) => {
                                println!("Could not parse audio info with error: {:#?}", e);
                                return;
                            }
                        };

                        user_data.raw_info = Some(audio_info);
                        println!("Finished format change and parsed format info.");
                    }
                    ParamType::Buffers => {
                        //TODO: Hanldle buffer configuration
                    }
                    _ => {
                        println!("Received invalid parameter type: {:#?}", param_id);
                        return;
                    }
                }
            });
        // audio_sink.connect(spa::utils::Direction::Input, None, )

        self.audio_sink = Some(audio_sink);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
}
