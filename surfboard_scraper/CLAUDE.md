# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Surfboard Scraper is a Rust application that generates surf report displays for e-ink screens. It fetches surf data from the Surfline API and renders visual reports as images that are uploaded to AWS S3 for consumption by Raspberry Pi devices with e-ink displays.

## Common Development Commands

### Building and Testing
- `make test` - Run all tests
- `cargo test` - Run tests (alternative to make)
- `make build` - Cross-compile for ARM (Raspberry Pi target)
- `make build CARGO_PROFILE=release` - Build optimized release version
- `make clean` - Clean build artifacts

### Cross-compilation Setup (for Raspberry Pi deployment)
The project uses cross-compilation to build ARM binaries:
```bash
brew install llvm arm-linux-gnueabihf-binutils
cargo install cross@0.2.4
```

### Deployment
- `make package` - Create deployment package (requires release build)
- `make deploy-scraper` - Deploy to Raspberry Pi at 192.168.4.79

### Testing Screen Rendering
Individual screen configurations can be tested by creating PNG outputs:
```rust
config.draw_to_png("test_output.png").await?
```

## Code Architecture

### Core Components

**Configuration System** (`device_config.rs`):
- `Configuration` struct defines screens and screensaver settings
- `ScreenConfiguration` maps screen types to parameters and S3 URLs
- Configuration loaded from `deploy/config.json`

**Screen System** (`screen.rs`):
- `Screen` trait defines the interface for all screen types
- `ScreenIdentifier` enum: `SurfReport24h`, `SurfReportWeek`, `ScreenSaver`
- Each screen type implements parameter parsing, data fetching, and rendering

**Screen Types**:
- `SurfReport24h` - 24-hour surf forecast display
- `SurfReportWeek` - Weekly surf forecast display  
- `ScreenSaver` - Static image display from local files

**Surfline API Integration** (`surfline_types/`):
- Type definitions for Surfline API responses
- Modules: conditions, spot_details, tide, wave, weather, wind

**Rendering Pipeline**:
- Uses `embedded-graphics` for drawing operations
- Targets e-ink displays (800x480 tri-color)
- Outputs QOI format images for efficient storage/transmission
- PNG output available for testing via simulator

### Data Flow

1. Main loop reads configuration from `deploy/config.json`
2. For each screen configuration:
   - Parse parameters based on screen type
   - Fetch surf data from Surfline API
   - Render screen to QOI format bytes
   - Upload to configured S3 bucket/path
3. Upload configuration file to S3
4. Sleep for 3 hours before next cycle

### Key Dependencies

- `cross` - Cross-compilation for ARM targets
- `embedded-graphics` - Graphics primitives for embedded displays
- `epd-waveshare` - E-ink display driver support
- `aws-sdk-s3` - S3 client for image uploads
- `reqwest` - HTTP client for Surfline API calls
- `image` - Image processing and format conversion

### Testing

Tests are located in `tests/` directory:
- `test_device_config.rs` - Configuration parsing tests
- `test_draw_screens.rs` - Screen rendering tests
- Test data images in `tests/data/`

### Formatting

Code uses rustfmt with max_width = 120 (configured in `rustfmt.toml`).