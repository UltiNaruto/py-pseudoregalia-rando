use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyRuntimeError;

use pseudoregalia_rando;

#[pyfunction]
#[text_signature = "(config_json, game_path, /)"]
fn patch_game(config_json: String, game_path: String, py: Python) -> PyResult<()> {
    py.allow_threads(move || {
        pseudoregalia_rando::patch_from_config(&config_json, game_path.into())
            .map_err(|e| PyRuntimeError::new_err(e))?;

        Ok(())
    })
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(patch_game, m)?)?;

    Ok(())
}