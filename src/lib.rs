extern crate bliss_audio as original_bliss;
use original_bliss::AnalysisIndex;
use original_bliss::Song as BlissSong;
use original_bliss::decoder::ffmpeg::FFmpegDecoder as Decoder;
use original_bliss::decoder::Decoder as DecoderTrait;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::path::Path;

#[pyclass]
pub struct Song {
    inner: BlissSong,
}

#[pymethods]
// TODO with syntax?
// TODO exceptions ?
impl Song {
    #[getter]
    fn title(&self) -> Option<String> {
        self.inner.title.to_owned()
    }

    #[getter]
    fn path(&self) -> String {
        self.inner.path.to_owned().to_string_lossy().to_string()
    }

    #[getter]
    fn artist(&self) -> Option<String> {
        self.inner.artist.to_owned()
    }

    #[getter]
    fn album(&self) -> Option<String> {
        self.inner.album.to_owned()
    }

    #[getter]
    fn duration(&self) -> f64 {
        self.inner.duration.to_owned().as_secs_f64()
    }

    #[getter]
    fn track_number(&self) -> Option<i32> {
        self.inner.track_number.to_owned()
    }

    #[getter]
    fn disc_number(&self) -> Option<i32> {
        self.inner.disc_number.to_owned()
    }

    #[getter]
    fn genre(&self) -> Option<String> {
        self.inner.genre.to_owned()
    }

    #[getter]
    fn analysis(&self) -> Vec<f32> {
        self.inner.analysis.as_vec()
    }

    #[getter]
    fn analysis_dict(&self) -> HashMap<String, f32> {
        HashMap::from([
            (
                String::from("tempo"),
                self.inner.analysis[AnalysisIndex::Tempo],
            ),
            (String::from("zcr"), self.inner.analysis[AnalysisIndex::Zcr]),
            (
                String::from("mean_spectral_centroid"),
                self.inner.analysis[AnalysisIndex::MeanSpectralCentroid],
            ),
            (
                String::from("std_spectral_centroid"),
                self.inner.analysis[AnalysisIndex::StdDeviationSpectralCentroid],
            ),
            (
                String::from("mean_spectral_rolloff"),
                self.inner.analysis[AnalysisIndex::MeanSpectralRolloff],
            ),
            (
                String::from("std_spectral_rolloff"),
                self.inner.analysis[AnalysisIndex::StdDeviationSpectralRolloff],
            ),
            (
                String::from("mean_spectral_flatness"),
                self.inner.analysis[AnalysisIndex::MeanSpectralFlatness],
            ),
            (
                String::from("std_spectral_flatness"),
                self.inner.analysis[AnalysisIndex::StdDeviationSpectralFlatness],
            ),
            (
                String::from("mean_loudness"),
                self.inner.analysis[AnalysisIndex::MeanLoudness],
            ),
            (
                String::from("std_loudness"),
                self.inner.analysis[AnalysisIndex::StdDeviationLoudness],
            ),
            (
                String::from("chroma1"),
                self.inner.analysis[AnalysisIndex::Chroma1],
            ),
            (
                String::from("chroma2"),
                self.inner.analysis[AnalysisIndex::Chroma2],
            ),
            (
                String::from("chroma3"),
                self.inner.analysis[AnalysisIndex::Chroma3],
            ),
            (
                String::from("chroma4"),
                self.inner.analysis[AnalysisIndex::Chroma4],
            ),
            (
                String::from("chroma5"),
                self.inner.analysis[AnalysisIndex::Chroma5],
            ),
            (
                String::from("chroma6"),
                self.inner.analysis[AnalysisIndex::Chroma6],
            ),
            (
                String::from("chroma7"),
                self.inner.analysis[AnalysisIndex::Chroma7],
            ),
            (
                String::from("chroma8"),
                self.inner.analysis[AnalysisIndex::Chroma8],
            ),
            (
                String::from("chroma9"),
                self.inner.analysis[AnalysisIndex::Chroma9],
            ),
            (
                String::from("chroma10"),
                self.inner.analysis[AnalysisIndex::Chroma10],
            ),
            (
                String::from("chroma11"),
                self.inner.analysis[AnalysisIndex::Chroma11],
            ),
            (
                String::from("chroma12"),
                self.inner.analysis[AnalysisIndex::Chroma12],
            ),
            (
                String::from("chroma13"),
                self.inner.analysis[AnalysisIndex::Chroma13],
            ),
        ])
    }

    #[new]
    fn new(path: &str) -> Self {
        Song {
            inner: Decoder::song_from_path(Path::new(path)).unwrap(),
        }
    }
}

#[pymodule]
fn bliss_audio(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Song>()?;

    Ok(())
}
