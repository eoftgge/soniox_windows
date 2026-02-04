use crate::errors::SonioxWindowsErrors;
use crate::soniox::MODEL;
use crate::settings::SettingsApp;
use crate::types::soniox::{SonioxTranscriptionRequest, SonioxTranslationObject};
use wasapi::{DeviceEnumerator, Direction, initialize_mta};

pub(crate) fn create_request(
    settings: &'_ SettingsApp,
) -> Result<SonioxTranscriptionRequest<'_>, SonioxWindowsErrors> {
    initialize_mta().ok()?;
    let enumerator = DeviceEnumerator::new()?;
    let device = enumerator.get_default_device(&Direction::Render)?;
    let audio_client = device.get_iaudioclient()?;
    let format = audio_client.get_mixformat()?;
    let sample_rate = format.get_samplespersec();
    let channels = format.get_nchannels();
    let mut request = SonioxTranscriptionRequest {
        api_key: settings.api_key(),
        model: MODEL,
        audio_format: "pcm_s16le",
        sample_rate: Some(sample_rate),
        num_channels: Some(channels as u32),
        context: Some(settings.context()),
        language_hints: settings.language_hints(),
        enable_speaker_diarization: Some(settings.enable_speakers()),
        ..Default::default()
    };
    if settings.enable_translate {
        request.translation = Some(SonioxTranslationObject {
            r#type: "one_way",
            target_language: Some(settings.target_language()),
            ..Default::default()
        });
    }

    Ok(request)
}
