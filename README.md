# rawler-py

Python bindings for the [rawler](https://crates.io/crates/rawler) Rust crate — high-performance RAW image decoding with NumPy output.


## Installation

```bash
pip install rawler-py
```

## Usage

```python
import rawler_py
from pathlib import Path

# Use context manager for automatic resource cleanup
with rawler_py.RawImage.open("path_to_raw") as img:
    # Basic Metadata
    print(f"{img.make} {img.model} | {img.width}x{img.height}")
    print(f"Orientation: {img.orientation} | Bits: {img.bps}")
    
    # Sensor Parameters
    print(f"CFA Pattern: {img.cfa_pattern}")
    print(f"White Balance Coeffs: {img.wb_coeffs}")
    print(f"Black Level: {img.blacklevel}") # {'levels': [...], 'width': 6, 'height': 6, 'cpp': 1}
    print(f"White Level: {img.whitelevel}")
    
    # Area definitions
    if img.crop_area:
        print(f"Crop Area: {img.crop_area}") # (x, y, w, h)

    # Get raw sensor data as NumPy arrays
    data = img.raw_data()          # uint16, shape (height, width)
    f32_data = img.raw_data_f32()  # float32
    
    # Automatically apply sensor crop/active area
    cropped = img.cropped_raw_data() 
```

## License

LGPL-2.1 — same as the underlying rawler crate.
