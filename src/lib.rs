use numpy::ndarray::Array2;
use numpy::PyArray2;
use pyo3::prelude::*;

#[pyclass]
pub struct RawImage {
    inner: rawler::RawImage,
}

#[pymethods]
impl RawImage {
    #[staticmethod]
    fn open(path: &str) -> PyResult<Self> {
        let inner = rawler::decode_file(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(Self { inner })
    }

    #[getter]
    fn width(&self) -> usize {
        self.inner.width
    }

    #[getter]
    fn height(&self) -> usize {
        self.inner.height
    }

    #[getter]
    fn bps(&self) -> usize {
        self.inner.bps
    }

    #[getter]
    fn cpp(&self) -> usize {
        self.inner.cpp
    }

    #[getter]
    fn make(&self) -> &str {
        &self.inner.make
    }

    #[getter]
    fn model(&self) -> &str {
        &self.inner.model
    }

    #[getter]
    fn clean_make(&self) -> &str {
        &self.inner.clean_make
    }

    #[getter]
    fn clean_model(&self) -> &str {
        &self.inner.clean_model
    }

    #[getter]
    fn wb_coeffs(&self) -> [f32; 4] {
        self.inner.wb_coeffs
    }

    fn raw_data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<u16>>> {
        match &self.inner.data {
            rawler::RawImageData::Integer(data) => {
                let w = self.inner.width * self.inner.cpp;
                let h = self.inner.height;
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
        let data = self.inner.data.as_f32();
        let w = self.inner.width * self.inner.cpp;
        let h = self.inner.height;
        let arr = Array2::from_shape_vec((h, w), data.into_owned())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(PyArray2::from_owned_array(py, arr))
    }

    #[getter]
    fn active_area(&self) -> Option<(usize, usize, usize, usize)> {
        self.inner.active_area.map(|r| (r.p.x, r.p.y, r.d.w, r.d.h))
    }

    #[getter]
    fn crop_area(&self) -> Option<(usize, usize, usize, usize)> {
        self.inner.crop_area.map(|r| (r.p.x, r.p.y, r.d.w, r.d.h))
    }

    fn cropped_raw_data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<u16>>> {
        let area = self.inner.crop_area
            .or(self.inner.active_area)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("no crop/active area defined"))?;

        match &self.inner.data {
            rawler::RawImageData::Integer(data) => {
                let full_w = self.inner.width * self.inner.cpp;
                let x = area.p.x * self.inner.cpp;
                let w = area.d.w * self.inner.cpp;
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
        format!(
            "RawImage({} {}, {}x{}, {}bps)",
            self.inner.clean_make, self.inner.clean_model, self.inner.width, self.inner.height, self.inner.bps
        )
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
    Ok(())
}
