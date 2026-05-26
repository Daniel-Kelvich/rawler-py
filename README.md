# rawler-py

Python bindings for the [rawler](https://crates.io/crates/rawler) Rust crate — fast RAW image decoding with numpy output.

Supports CR2, CR3, NEF, ARW, RAF, DNG, ORF, RW2, PEF, and more.

## Installation

```bash
pip install rawler-py
```

## Usage

```python
import rawler_py

img = rawler_py.decode("photo.CR3")

# Metadata
img.width        # 8192
img.height       # 5464
img.bps          # 14 (bits per sample)
img.cpp          # 1 (components per pixel)
img.make         # "Canon"
img.model        # "Canon EOS R5"
img.clean_make   # "Canon"
img.clean_model  # "EOS R5"
img.wb_coeffs    # [1.234, 1.0, 2.345, 1.0]
img.active_area  # (x, y, w, h) or None
img.crop_area    # (x, y, w, h) or None

# Raw sensor data as numpy arrays
data = img.raw_data()          # uint16, shape (height, width)
data = img.raw_data_f32()      # float32, same shape
data = img.cropped_raw_data()  # uint16, cropped to active/crop area
```

## License

LGPL-2.1 — same as the underlying rawler crate.
