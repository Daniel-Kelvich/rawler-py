use numpy::ndarray::Array2;
use numpy::PyArray2;
use pyo3::prelude::*;

#[pyclass]
pub struct RawImage {
    inner: Option<rawler::RawImage>,
}

impl RawImage {
    fn get_inner(&self) -> PyResult<&rawler::RawImage> {
        self.inner
            .as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("RawImage is closed"))
    }
}

#[pymethods]
impl RawImage {
    #[staticmethod]
    fn open(path: &str) -> PyResult<Self> {
        let inner = rawler::decode_file(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(Self { inner: Some(inner) })
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __exit__(
        &mut self,
        _exc_type: Option<Bound<'_, PyAny>>,
        _exc_value: Option<Bound<'_, PyAny>>,
        _traceback: Option<Bound<'_, PyAny>>,
    ) {
        self.close();
    }

    fn close(&mut self) {
        self.inner = None;
    }

    fn is_closed(&self) -> bool {
        self.inner.is_none()
    }

    #[getter]
    fn width(&self) -> PyResult<usize> {
        Ok(self.get_inner()?.width)
    }

    #[getter]
    fn height(&self) -> PyResult<usize> {
        Ok(self.get_inner()?.height)
    }

    #[getter]
    fn bps(&self) -> PyResult<usize> {
        Ok(self.get_inner()?.bps)
    }

    #[getter]
    fn cpp(&self) -> PyResult<usize> {
        Ok(self.get_inner()?.cpp)
    }

    #[getter]
    fn make(&self) -> PyResult<String> {
        Ok(self.get_inner()?.make.clone())
    }

    #[getter]
    fn model(&self) -> PyResult<String> {
        Ok(self.get_inner()?.model.clone())
    }

    #[getter]
    fn clean_make(&self) -> PyResult<String> {
        Ok(self.get_inner()?.clean_make.clone())
    }

    #[getter]
    fn clean_model(&self) -> PyResult<String> {
        Ok(self.get_inner()?.clean_model.clone())
    }

    #[getter]
    fn cfa_pattern(&self) -> PyResult<String> {
        Ok(self.get_inner()?.camera.cfa.name.clone())
    }

    #[getter]
    fn cropped_cfa_pattern(&self) -> PyResult<Option<String>> {
        let inner = self.get_inner()?;
        let area = match inner.crop_area.or(inner.active_area) {
            Some(a) => a,
            None => return Ok(None),
        };
        let shifted = inner.camera.cfa.shift(area.p.x, area.p.y);
        Ok(Some(shifted.name))
    }

    #[getter]
    fn wb_coeffs(&self) -> PyResult<[f32; 4]> {
        Ok(self.get_inner()?.wb_coeffs)
    }

    fn raw_data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<u16>>> {
        let inner = self.get_inner()?;
        match &inner.data {
            rawler::RawImageData::Integer(data) => {
                let w = inner.width * inner.cpp;
                let h = inner.height;
                let arr = Array2::from_shape_vec((h, w), data.clone())
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                Ok(PyArray2::from_owned_array(py, arr))
            }
            rawler::RawImageData::Float(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "raw data is f32, use raw_data_f32()",
            )),
        }
    }

    fn raw_data_f32<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f32>>> {
        let inner = self.get_inner()?;
        let data = inner.data.as_f32();
        let w = inner.width * inner.cpp;
        let h = inner.height;

        let arr = Array2::from_shape_vec((h, w), data.into_owned())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(PyArray2::from_owned_array(py, arr))
    }

    #[getter]
    fn active_area(&self) -> PyResult<Option<(usize, usize, usize, usize)>> {
        Ok(self.get_inner()?.active_area.map(|r| (r.p.x, r.p.y, r.d.w, r.d.h)))
    }

    #[getter]
    fn crop_area(&self) -> PyResult<Option<(usize, usize, usize, usize)>> {
        Ok(self.get_inner()?.crop_area.map(|r| (r.p.x, r.p.y, r.d.w, r.d.h)))
    }

    #[getter]
    fn orientation(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_inner()?.orientation))
    }

    #[getter]
    fn whitelevel(&self) -> PyResult<Vec<u32>> {
        Ok(self.get_inner()?.whitelevel.0.clone())
    }

    #[getter]
    fn blacklevel<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
        let inner = self.get_inner()?;
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item(
            "levels",
            inner
                .blacklevel
                .levels
                .iter()
                .map(|r| r.as_f32())
                .collect::<Vec<f32>>(),
        )?;
        dict.set_item("width", inner.blacklevel.width)?;
        dict.set_item("height", inner.blacklevel.height)?;
        dict.set_item("cpp", inner.blacklevel.cpp)?;
        Ok(dict)
    }

    #[getter]
    fn blackareas(&self) -> PyResult<Vec<(usize, usize, usize, usize)>> {
        Ok(self
            .get_inner()?
            .blackareas
            .iter()
            .map(|r| (r.p.x, r.p.y, r.d.w, r.d.h))
            .collect())
    }

    #[getter]
    fn photometric(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_inner()?.photometric))
    }

    #[getter]
    fn xyz_to_cam(&self) -> PyResult<[[f32; 3]; 4]> {
        Ok(self.get_inner()?.xyz_to_cam)
    }

    #[getter]
    fn color_matrix<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
        let inner = self.get_inner()?;
        let dict = pyo3::types::PyDict::new(py);
        for (ill, matrix) in &inner.color_matrix {
            dict.set_item(format!("{:?}", ill), matrix)?;
        }
        Ok(dict)
    }

    #[getter]
    fn dng_tags<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
        let inner = self.get_inner()?;
        let dict = pyo3::types::PyDict::new(py);
        for (tag, value) in &inner.dng_tags {
            dict.set_item(tag, format!("{:?}", value))?;
        }
        Ok(dict)
    }

    fn cropped_raw_data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<u16>>> {
        let inner = self.get_inner()?;
        let area = inner
            .crop_area
            .or(inner.active_area)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("no crop/active area defined"))?;

        match &inner.data {
            rawler::RawImageData::Integer(data) => {
                let full_w = inner.width * inner.cpp;
                let x = area.p.x * inner.cpp;
                let w = area.d.w * inner.cpp;
                let h = area.d.h;
                let mut cropped = Vec::with_capacity(w * h);
                for row in area.p.y..area.p.y + h {
                    let start = row * full_w + x;
                    cropped.extend_from_slice(&data[start..start + w]);
                }
                let arr = Array2::from_shape_vec((h, w), cropped)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                Ok(PyArray2::from_owned_array(py, arr))
            }
            rawler::RawImageData::Float(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "raw data is f32, use raw_data_f32()",
            )),
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            Some(inner) => format!(
                "RawImage({} {}, {}x{}, {}bps)",
                inner.clean_make, inner.clean_model, inner.width, inner.height, inner.bps
            ),
            None => "RawImage(Closed)".to_string(),
        }
    }
}

#[pyfunction]
fn decode(path: &str) -> PyResult<RawImage> {
    RawImage::open(path)
}

#[pymodule]
fn rawler_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RawImage>()?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
