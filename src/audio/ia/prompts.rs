pub const AUDIO_CLASSIFICATION_SYSTEM_PROMPT: &str =
    "Eres un experto en produccion musical y clasificacion de audio. Responde unicamente con JSON valido, sin texto adicional.";

pub const SHARED_JSON_FIELD_INSTRUCTIONS: &str = r#"- "nombre_archivo_base": Un titulo corto y descriptivo para el sample, en ingles, en minusculas y usando espacios. Ej: "deep kick 808", "sad guitar melody".
- "tags": Array de strings con etiquetas descriptivas en INGLES (ej: "melodic", "dark", "808", "lo-fi").
- "tags_es": Array de strings con las mismas etiquetas que 'tags' pero traducidas al ESPANOL.
- "tipo": String, debe ser "one shot" o "loop".
- "genero": Array de strings con generos musicales en INGLES (ej: "hip hop", "trap", "electronic").
- "emocion": Array de strings con emociones que evoca en INGLES (ej: "energetic", "sad", "chill").
- "emocion_es": Array de strings con las mismas emociones que 'emocion' pero traducidas al ESPANOL.
- "instrumentos": Array de strings con los instrumentos principales que detectes en INGLES (ej: "guitar", "piano", "synth", "drums").
- "artista_vibes": Array de strings con nombres de artistas que tienen un estilo similar.
- "descripcion_corta": Una descripcion muy breve (10-15 palabras) en INGLES.
- "descripcion_corta_es": La misma 'descripcion_corta' traducida al ESPANOL.
- "descripcion": Una descripcion detallada (30-50 palabras) en INGLES.
- "descripcion_es": La misma 'descripcion' traducida al ESPANOL.
- "carpeta_primaria": Elige UNA de estas carpetas principales segun el tipo de audio: "Drums", "Loops", "Samples", "FX", "Instruments", "Vocals".
- "carpeta_secundaria": Subcarpeta dentro de carpeta_primaria. OBLIGATORIO, NUNCA null ni vacio. Si no encaja, usa "General"."#;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AudioExtractionContext {
    pub source_title: Option<String>,
    pub source_artist: Option<String>,
    pub destination_title: Option<String>,
    pub destination_artist: Option<String>,
    pub element_type: Option<String>,
    pub vote_count: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AudioAnalysisPromptInput {
    pub original_filename: String,
    pub user_description: String,
    pub user_tags: Vec<String>,
    pub bpm: Option<f32>,
    pub musical_key: Option<String>,
    pub scale: Option<String>,
    pub duration_seconds: Option<f32>,
    pub upload_origin: Option<String>,
    pub extraction_context: Option<AudioExtractionContext>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MetadataCorrectionPromptInput {
    pub current_metadata_json: String,
    pub title: String,
    pub instructions: String,
    pub bpm: Option<f32>,
    pub musical_key: Option<String>,
}

#[must_use]
pub fn build_analysis_prompt(input: &AudioAnalysisPromptInput) -> String {
    let mut parts = Vec::new();

    if let Some(extraction) = input.extraction_context.as_ref() {
        let mut fragments = Vec::new();
        if let Some(source_title) = non_empty(&extraction.source_title) {
            let source_artist = non_empty(&extraction.source_artist);
            fragments.push(match source_artist {
                Some(artist) => format!("Sampled from \"{source_title}\" by {artist}"),
                None => format!("Sampled from \"{source_title}\""),
            });
        }
        if let Some(destination_title) = non_empty(&extraction.destination_title) {
            let destination_artist = non_empty(&extraction.destination_artist);
            fragments.push(match destination_artist {
                Some(artist) => format!("Used in \"{destination_title}\" by {artist}"),
                None => format!("Used in \"{destination_title}\""),
            });
        }
        if let Some(element_type) = non_empty(&extraction.element_type) {
            fragments.push(format!("Element: {element_type}"));
        }
        if let Some(vote_count) = extraction.vote_count.filter(|count| *count > 0) {
            fragments.push(format!("Confidence: {vote_count} votes"));
        }
        if !fragments.is_empty() {
            parts.push(format!(
                "{}. This is a sample extracted from another track - analyze considering origin genre/style.",
                fragments.join(" | ")
            ));
        }
    }

    if parts.is_empty() {
        parts.push(format!(
            "El archivo se subio con este nombre: \"{}\".",
            input.original_filename.trim()
        ));
    }

    if let Some(description) = non_empty_str(&input.user_description) {
        parts.push(format!(
            "El usuario ha descrito el audio de esta manera: \"{description}\"."
        ));
    }

    if !input.user_tags.is_empty() {
        let tags = input
            .user_tags
            .iter()
            .map(|tag| format!("#{tag}"))
            .collect::<Vec<_>>()
            .join(", ");
        parts.push(format!("El usuario ha colocado los siguientes tags: {tags}."));
    }

    if let Some(bpm) = input.bpm.filter(|value| *value > 0.0) {
        parts.push(format!("El archivo tiene un BPM de {bpm:.1}."));
    }

    if let Some(musical_key) = non_empty(&input.musical_key) {
        let scale_suffix = non_empty(&input.scale)
            .map(|scale| format!(" {scale}"))
            .unwrap_or_default();
        parts.push(format!(
            "La tonalidad detectada es {musical_key}{scale_suffix}."
        ));
    }

    if let Some(duration) = input.duration_seconds.filter(|value| *value > 0.0) {
        parts.push(format!("Dura {duration:.1} segundos."));
    }

    if let Some(upload_origin) = non_empty(&input.upload_origin) {
        parts.push(format!(
            "El archivo fue subido desde la siguiente ruta de carpetas: \"{upload_origin}\". Usa los nombres de las carpetas como pistas contextuales para inferir genero, artista y estilo."
        ));
    }

    let context = parts.join(" ");
    format!(
        "Analiza este audio. {context} Tu tarea es generar UNICAMENTE un objeto JSON valido con la siguiente estructura. Se creativo y preciso. NO incluyas en tu respuesta los campos puramente tecnicos (bpm, tonalidad, escala), ya que esos se anadiran despues. Tu respuesta DEBE ser solo el JSON.\n\n{SHARED_JSON_FIELD_INSTRUCTIONS}"
    )
}

#[must_use]
pub fn append_transcription_context(base_prompt: &str, transcription: &str) -> String {
    let excerpt: String = transcription.trim().chars().take(3_000).collect();

    format!(
        "{base_prompt}\n\nContexto adicional obtenido por transcripcion de audio (Whisper):\n\"{excerpt}\"\n\nDebes considerar ese contexto para inferir mejor emocion, genero, instrumentos y artista_vibes. Si hay poco contenido verbal, responde igual con un JSON valido apoyandote en el resto del contexto."
    )
}

#[must_use]
pub fn build_correction_prompt(input: &MetadataCorrectionPromptInput) -> String {
    let bpm_context = input
        .bpm
        .map(|bpm| format!("BPM: {bpm:.1}"))
        .unwrap_or_default();
    let key_context = non_empty(&input.musical_key)
        .map(|musical_key| format!("Key: {musical_key}"))
        .unwrap_or_default();

    format!(
        "Tienes un sample musical con el titulo \"{}\". {} {}\n\nSu metadata actual generada por IA es:\n```json\n{}\n```\n\nEl administrador solicita la siguiente CORRECCION:\n\"{}\"\n\nTu tarea es corregir la metadata segun las instrucciones. Manten la misma estructura JSON exacta. Si las instrucciones mencionan un titulo o nombre correcto, actualiza \"nombre_archivo_base\" con ese nombre en ingles minusculas. Si mencionan genero, artista, emocion u otros campos, actualiza los campos correspondientes. Los campos que NO se mencionan en las instrucciones deben mantenerse IGUALES que la metadata actual.\n\nIMPORTANTE: Responde UNICAMENTE con un JSON valido que tenga EXACTAMENTE estos campos:\n{}",
        input.title.trim(),
        bpm_context,
        key_context,
        input.current_metadata_json.trim(),
        input.instructions.trim(),
        SHARED_JSON_FIELD_INSTRUCTIONS
    )
}

fn non_empty(value: &Option<String>) -> Option<&str> {
    value.as_deref().map(str::trim).filter(|value| !value.is_empty())
}

fn non_empty_str(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
#[path = "prompts/tests.rs"]
mod tests;