# surfboard_scraper

The application fetches surf data from Surfline API and renders visual reports as images that are uploaded to AWS S3 for consumption by Raspberry Pi devices with e-ink
  displays.

  Architecture Overview

  Main Components

  1. Configuration System (device_config.rs)
  - Configuration struct manages multiple screen configurations
  - ScreenConfiguration maps screen types to parameters and S3 URLs
  - Supports both regular screens and screensaver functionality

  2. Screen System (screen.rs)
  - Screen trait defines common interface for all screen types
  - ScreenIdentifier enum: SurfReport24h, SurfReportWeek
  - Each screen type handles parameter parsing, data fetching, and rendering

  3. Screen Implementations
  - 24h Surf Report (surf_report_24h/): Short-term forecast display
  - Weekly Surf Report (surf_report_week/): Extended forecast view
  - Each has separate data.rs and draw.rs modules

  4. Surfline API Integration (surfline_types/)
  - Complete type definitions for Surfline API responses
  - Modules for: conditions, spot_details, tide, wave, weather, wind

  5. Rendering Pipeline
  - Uses embedded-graphics for drawing operations
  - Targets tri-color e-ink displays (800x480)
  - Outputs QOI format for efficient storage/transmission
  - PNG output available for testing

  Data Flow

  1. Main Loop (main.rs):
    - Reads configuration from deploy/config.json
    - For each screen: parse params → fetch surf data → render → upload to S3
    - Uploads configuration file to S3
    - Sleeps for 3 hours before next cycle
  2. Cross-compilation Setup: Builds ARM binaries for Raspberry Pi deployment
  3. AWS Integration: Uploads rendered images and config to S3 with public read access

  Key Dependencies

  - Cross-compilation: cross for ARM targets
  - Graphics: embedded-graphics, epd-waveshare for e-ink displays
  - Cloud: aws-sdk-s3 for image storage
  - HTTP: reqwest for Surfline API calls
  - Async: tokio runtime

  Testing & Development

  - Test suite in tests/ with sample data images
  - PNG output mode for development/debugging
  - Makefile with common commands (make test, make build, make deploy-scraper)

## Compiling for raspberry pi

```bash
brew install llvm

export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
export TARGET_CC=$(which clang)

brew install arm-linux-gnueabihf-binutils

cargo install cross@0.2.4
CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --target armv7-unknown-linux-musleabihf
```
