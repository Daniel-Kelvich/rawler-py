# rawler-py

Python bindings for the [rawler](https://github.com/dnglab/dnglab) Rust crate — fast RAW image decoding with zero-copy numpy array output.

## Installation

```bash
pip install maturin
git clone <this-repo>
cd rawler-py
maturin develop --release
```

## Usage

```python
import rawler_py

# Decode a RAW file
img = rawler_py.decode("photo.CR3")
# or
img = rawler_py.RawImage.open("photo.CR3")

# Metadata
print(img)              # RawImage(Canon EOS R5, 8192x5464, 14bps)
print(img.make)         # "Canon"
print(img.model)        # "Canon EOS R5"
print(img.clean_make)   # "Canon"
print(img.clean_model)  # "EOS R5"
print(img.width)        # 8192
print(img.height)       # 5464
print(img.bps)          # 14 (bits per sample)
print(img.cpp)          # 1 (components per pixel, 1 for bayer, 3 for RGB)
print(img.wb_coeffs)    # [1.234, 1.0, 2.345, 1.0] (RGBE white balance)

# Raw sensor data as numpy array
data = img.raw_data()        # np.ndarray[uint16], shape (height, width)
data = img.raw_data_f32()    # np.ndarray[float32], same shape
```

## Supported formats

All formats supported by rawler 0.7.2: CR2, CR3, NEF, ARW, RAF, DNG, ORF, RW2, PEF, and more.
