# Dust Language Full Build & Release Plan

**Executive Summary:** This report presents a comprehensive plan to discover, build, test, package, and maintain the Dust programming language from the ground up. We cover all phases: identifying official repos and specs, setting up cross-platform environments (Linux/Debian, macOS, Windows on x86_64 and ARM64), choosing compilers and tools, retrieving source code, designing the build system, managing dependencies, compiling, packaging (deb/rpm/zip/Docker/NuGet), and deploying via CI/CD. We also detail unit/integration/fuzz testing, profiling and optimization, multi-arch builds, release versioning, documentation generation, onboarding guides, security auditing, and ongoing maintenance. Each section includes concrete commands, configs, scripts, tables comparing options, and Mermaid diagrams for workflows. A 12-week timeline with milestones and checklists is provided to guide implementation. All recommendations prioritize official Dust sources and industry best practices.

## Discovery & Requirements

- **Dust Variants:** There appear to be multiple Dust projects. For example, **DustLanguage/Dust** is a C#/.NET Core implementation (MIT license), and **kadir014/Dust** is a reference implementation in C with a Python build script. There is also a theoretical **Dust DPL** specification (for unified classical/quantum computing). We assume our focus is the DustLanguage project but can learn from others. 
- **Design Goals:** Early docs note that Dust targets embedded systems with macro-based extensibility and strict module boundaries. While our build plan is language-agnostic, understanding Dust’s embedded/macro design helps (e.g. no fixed keywords, compile-time macros). 
- **Target Platforms & Architectures:** We must support Linux (Debian/Ubuntu), macOS, and Windows on both x86_64 and ARM64 (Apple Silicon, AWS Graviton). Deployment targets include local installs, Docker containers, and cloud services (AWS/GCP). This informs toolchain choices (e.g. .NET Core, GCC, cross-compilers) and CI matrix strategies. 

## Prerequisites & Environment Setup

- **System Dependencies:** Install Git, required SDKs, and build tools on each OS. For example, on Ubuntu:
  ```bash
  sudo apt-get update
  sudo apt-get install -y git wget curl build-essential python3 python3-pip
  ```
  Then install .NET SDK (for C#) and tools. On Ubuntu 26.04+, .NET SDK is in the package feed, e.g.:
  ```bash
  sudo apt-get install -y dotnet-sdk-10.0   # or current version
  ```
  On macOS, use Homebrew: `brew install --cask dotnet-sdk` (or `brew install dotnet-sdk`). On Windows, download the .NET SDK installer from Microsoft or use `winget install Microsoft.DotNet.SDK.10`. Ensure Python 3.9+ is installed (via `apt`, `brew`, or the Windows installer) for any Python build scripts.
- **MinGW/Clang (for C Dust):** If building the C-based Dust variant, install a C compiler. On Ubuntu/Debian: `sudo apt-get install -y gcc g++`. On macOS: Xcode Command Line Tools (`xcode-select --install`). On Windows: install [MinGW-w64](http://mingw-w64.org/) or use Visual Studio Build Tools (C++ workload).
- **Rust/Node (if used):** Some Dust forks use Rust or other tooling (e.g. `dust-lang-draft`). If relevant, install Rust toolchain or Node.js via rustup/nvm.
- **Docker:** Install Docker Engine (incl. Docker CLI). Ensure [Docker Desktop](https://docs.docker.com/get-docker/) on Windows/macOS and `docker.io` on Linux. This allows building containers locally.
- **AWS/GCP CLI (optional):** For deployment scripts, install AWS CLI (`pip install awscli`) or Google Cloud SDK if using those clouds.

## Toolchain Selection & Build System Design

- **Languages/Compilers:** For DustLanguage (C#), we use .NET 7/8 (or latest) with the `dotnet` CLI. The Travis CI config for DustLanguage indicates .NET Core 3.0.101 was used; today use a modern LTS (e.g. .NET 7 or 10). For any C components (parser, runtime), use GCC/Clang or MSVC. If Python is part of the build (scripts, tests), use Python 3.9+.
- **Build Tool:** Use `dotnet msbuild`/`dotnet build` for C# projects. According to Microsoft, `dotnet build` is equivalent to `dotnet msbuild -restore`. The DustLanguage Travis CI uses:
  ```yaml
  language: csharp
  dotnet: 3.0.101
  solution: Dust.sln
  install: 
    - dotnet restore
  script:
    - dotnet build --configuration Release
    - dotnet test
  ```
  which restores packages and builds/tests the solution. We will follow this pattern with updated versions. For C projects, a common approach is a Makefile or CMake. The C Dust repo uses a `build.py` Python script for building and packaging. We may adopt a similar script or integrate with CMake.
- **Dependency Management:** 
  - *C#/.NET:* Dependencies are NuGet packages declared in `.csproj` files. `dotnet restore` retrieves them from NuGet (public or private feeds). Example: `dotnet restore Dust.sln`. For new dependencies, use `dotnet add package`.
  - *Python:* Use a `requirements.txt` or Poetry if Python is used. For example, `pip install -r requirements.txt`.
  - *C:* Manage libraries via system packages or tools like [vcpkg](https://github.com/Microsoft/vcpkg) or [Conan](https://conan.io) if many C/C++ deps. For a small compiler, minimize external deps.
- **Directory Layout:** Follow a standard structure, e.g.:
  ```
  dust/               # root
    src/              # source code (C#, C, etc.)
      DustCompiler/   # C# compiler project
      DustVM/         # C/C++ runtime or VM
    tests/            # unit/integration tests
    scripts/          # build and utility scripts
    docs/             # documentation, spec
    .github/workflows/ # CI config
    Dockerfile        # container setup
    ...
  ```
- **Configuration & Scripts:** Provide example `build.sh` (Linux) and `build.ps1` (PowerShell for Windows) to automate steps. For instance, a `build.sh` might contain:
  ```bash
  #!/bin/bash
  dotnet restore Dust.sln
  dotnet build Dust.sln --configuration Release
  dotnet test Dust.UnitTests/Dust.UnitTests.csproj
  ```
  Cite the Travis example build steps.

## Source Acquisition

- **Official Repos:** Clone the DustLanguage repository:
  ```bash
  git clone https://github.com/DustLanguage/Dust.git
  ```
  (This repo is MIT licensed and contains the compiler source). Also clone or fork other relevant Dust projects if needed for reference (e.g. `kadir014/Dust`, `dustlang/dustlang`, etc.).
- **Versioning:** Use Git tags and semantic versioning (v0.1.0, v1.0.0, etc.). If no existing tags, establish a version scheme. For releases, tag commits in Git and push to GitHub, then create a GitHub Release. We will automate releases in CI (see Release Management).
- **Dependencies (external):** For any submodules or third-party libraries, record their versions (e.g. in a `dependencies.md`). Use tagged releases if possible for reproducibility.

## Build System & Dependency Management

- **Build Configuration:** For C#, use the `.csproj` and `global.json` (if pinning SDK version) to define build settings. Ensure `LangVersion`, optimization flags, warnings as errors if desired. For C (if any), use a `Makefile` or CMakeLists.txt with appropriate flags (`-O2`, standards, etc.).
- **Dependency Isolation:** 
  - For .NET, use `dotnet restore` which uses a global NuGet cache. For CI, always clean or restore fresh to avoid stale caches.
  - For Python, use `venv` or `poetry` to isolate dependencies. Check in the `poetry.lock` or `requirements.txt`.
  - Use *Dependabot* or *Renovate* (GitHub Apps) to automatically track and update third-party dependencies.
- **Container Builds (optional):** For reproducible builds, use Docker. For example, a Dockerfile for building DustLanguage might use the official .NET SDK image:
  ```dockerfile
  FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
  WORKDIR /src
  COPY . .
  RUN dotnet restore
  RUN dotnet build -c Release
  
  FROM build AS publish
  WORKDIR /app
  RUN dotnet publish Dust.sln -c Release -o /app/publish
  ```
  This ensures a clean Linux x64 build environment. Later we’ll use multi-arch Docker (via `docker buildx`) to also produce ARM64 images.

## Compilation Steps & Commands

- **.NET Build:** Use `dotnet` CLI (works on all OS). For example:
  ```bash
  dotnet restore Dust.sln
  dotnet build Dust.sln --configuration Release
  dotnet test Dust.UnitTests/Dust.UnitTests.csproj --no-build --verbosity normal
  ```
  These steps restore NuGet packages, build all projects, and run unit tests. This aligns with the official Travis CI script.
- **C Build:** If using `build.py` (as in one Dust variant), run `python3 build.py` (with MinGW on Windows or gcc on Linux). If using Make/CMake:
  ```bash
  mkdir -p build && cd build
  cmake .. -DCMAKE_BUILD_TYPE=Release
  make -j$(nproc)
  ```
- **Cross-Compilation:** For ARM64 targets, either build on ARM hardware (GitHub Actions has `runs-on: ubuntu-22.04` for ARM; Azure Pipelines has `vmImage: macos-13-arm64`) or use cross-compilers. For C#: specify runtime IDs during publish (e.g. `-r linux-arm64` in `dotnet publish`). For Docker, use `docker buildx build --platform linux/arm64,linux/amd64`.

## Packaging & Distribution

- **Language Executables:** For .NET, consider packaging the compiler as a [Global Tool](https://learn.microsoft.com/en-us/dotnet/core/tools/global-tools) (a NuGet-based console app). For example, use `dotnet pack` to create a NuGet package and publish it. Users can then install with `dotnet tool install -g Dust`. Alternately, publish cross-platform binaries or scripts.
- **Archives/Installers:** Provide downloads for:
  - **Windows:** Zip archive or MSI installer containing the `DustCompiler.exe` and tools. E.g. use [WiX Toolset](https://wixtoolset.org/) for MSI.
  - **macOS:** `.pkg` installer or Homebrew formula.
  - **Linux:** DEB (for Debian/Ubuntu) and RPM (for RHEL/Fedora) packages. Use `fpm` or native packaging tools, e.g.:
    ```bash
    fpm -s dir -t deb -n dust -v 1.0.0 ./publish=/usr/local/dust
    ```
- **Container Images:** Publish Docker images (e.g. on Docker Hub) for easy deployment. Use multi-arch builds. For example:
  ```bash
  docker buildx build --platform linux/amd64,linux/arm64 -t dustlang/dust:latest --push .
  ```
- **NuGet/PyPI:** If components are libraries (e.g. a Dust runtime library), publish to NuGet (C#) or PyPI (Python) respectively.
- **Versioning:** Embed the version number from Git tags into builds (e.g. use a `VersionPrefix` in `.csproj` or auto-increment). Tag releases in GitHub and create GitHub Releases with changelogs.

## CI/CD Pipeline

- **CI Matrix:** Use GitHub Actions (or equivalent CI) with a matrix strategy for OS and architecture. Example GitHub workflow snippet:
  ```yaml
  jobs:
    build:
      strategy:
        matrix:
          os: [ubuntu-latest, macos-latest, windows-latest]
          arch: [x64, arm64]
      runs-on: ${{ matrix.os }}
      steps:
        - uses: actions/checkout@v3
        - name: Setup .NET
          uses: actions/setup-dotnet@v3
          with:
            dotnet-version: '10.0.x'
        - name: Build and Test
          run: |
            dotnet restore
            dotnet build --configuration Release
            dotnet test --no-build
  ```
  This runs on all OS/arch combos (GitHub provides ARM runners). Matrix strategies automatically spawn jobs.
- **CI Tools:** GitHub Actions is preferred (free for open-source, integrated). Alternatives: Travis CI (used historically), GitLab CI, Azure Pipelines, CircleCI, Jenkins. A comparison:

  | CI Provider        | Free for OSS | OS Support           | Cloud/On-premise   |
  |--------------------|--------------|----------------------|--------------------|
  | GitHub Actions     | Yes          | Linux/macOS/Windows  | GitHub-hosted      |
  | Travis CI          | Yes*         | Linux/macOS/Windows  | Cloud (limited quotas for OSS) |
  | GitLab CI/CD       | Yes          | Linux (macOS, Windows runners via self-host) | GitLab-hosted or self-hosted |
  | Azure Pipelines    | Yes (OSS)    | Linux/macOS/Windows  | Cloud |
  | CircleCI           | Yes (OSS)    | Linux/macOS/Windows  | Cloud |
  
  *(Travis free for OSS but slower recently). Choose GitHub Actions for tight integration and matrix builds. 
- **CD (Continuous Deployment):** Configure the pipeline to publish on successful builds of tagged commits: upload build artifacts, publish Docker image, create GitHub Release (using `actions/create-release@v1`), and push to NuGet or other registries. Encrypt credentials (e.g. NuGet API key) in CI secrets.

<div class="mermaid">
graph LR
  A[Commit to main] --> B[CI Build Matrix: restore/build/test on Ubuntu, Mac, Win]
  B --> C{Tests pass?}
  C -- No --> D[Notify devs]
  C -- Yes --> E[Package Artifacts (exe, deb, docker image)]
  E --> F[Publish to registries (NuGet, DockerHub) & GitHub Releases]
  F --> G[Deploy to AWS/GCP (optional)]
</div>

*Figure: Example CI/CD pipeline (multi-OS matrix build, test, package, release).*

## Testing Strategies

- **Unit Tests:** Create a suite of unit tests (e.g. using XUnit or NUnit for C#) under `Dust.UnitTests`. Automate via `dotnet test`. For the C compiler/interpreter, write tests in C or Python harnesses (as in `tests_run.py`).
- **Integration Tests:** Develop end-to-end tests that compile and run sample Dust programs, comparing output to expected. Include tests on all target OS/archs (via CI matrix).
- **Fuzz Testing:** Use fuzzers to find parser/compiler bugs. For example, integrate [AFL++](https://github.com/AFLplusplus/AFLplusplus) or [libFuzzer](https://llvm.org/docs/LibFuzzer.html) to mutate Dust programs and feed to the compiler/interpreter. This is an advanced step, but highly recommended for language robustness.
- **Test Coverage:** Use coverage tools (e.g. `coverlet` for .NET) to ensure critical code paths are exercised. Aim for high coverage on core language semantics.
- **Static Analysis:** Run analyzers: 
  - For C#: enable analyzers (Roslyn, SonarQube). 
  - For C/C++: use `clang-tidy` or `cppcheck`.
  - For dependencies: use [Dependabot](https://github.com/dependabot) or similar to check vulnerabilities.
- **Regression Testing:** Store failing cases in a `bugs/` or `regressions/` suite, and ensure fixes do not reintroduce issues.

## Performance Profiling & Optimization

- **Profiling Tools:** 
  - For C#: use [dotnet-counters](https://github.com/dotnet/diagnostics) or [PerfView](https://github.com/microsoft/perfview) on Windows, and [Linux perf](https://perf.wiki.kernel.org/index.php/Main_Page) or [dotnet-trace](https://github.com/dotnet/diagnostics) on Linux to identify CPU hotspots. 
  - For C/C++: use `gprof`, `perf`, or [Valgrind](https://valgrind.org/) to find slow paths or leaks.
- **Benchmarking:** Write benchmark tests (e.g. using [BenchmarkDotNet](https://benchmarkdotnet.org/) for .NET) to measure compiler throughput, runtime performance, memory usage. Optimize critical algorithms (e.g. parser speed, bytecode execution).
- **Build with Optimization:** Ensure Release builds are optimized (`-O2` or `/O2`), and consider producing both debug and optimized builds for performance comparisons. For ARM64, test on actual hardware to catch architecture-specific issues.
- **Continuous Profiling (optional):** Integrate periodic benchmarking (e.g. on CI or nightly) to detect performance regressions. Store results and trend over time.

## Cross-Platform & Multi-Arch Builds

- **CI Matrix:** As above, run builds on Ubuntu, macOS, Windows, on both x64 and ARM64. GitHub Actions supports `runs-on: macos-13-arm64` etc.
- **Conditional Steps:** In CI YAML, use the matrix to conditionally install tools. For example, on Windows use `choco install python3` if needed, on macOS use `brew`.
- **Cross-Compiling:** Use `dotnet publish -r <RID>` to produce executables for specific platforms. For example:
  ```bash
  dotnet publish -c Release -r ubuntu.22.04-arm64 --self-contained false
  ```
  This produces an ARM64 Linux binary. RIDs (Runtime Identifiers) lists are in .NET docs.
- **Docker Multi-Arch:** Use Docker Buildx for multi-arch images (see Packaging). Ensure base images match target OS/arch.
- **Validation:** Test the built artifacts on each platform. For example, run a “hello world” Dust program on Ubuntu ARM and verify output.

## Release Management

- **Versioning & Tags:** Use semantic versioning (Major.Minor.Patch). Tag releases in Git (`git tag v1.0.0`), push tags, and create GitHub Releases with notes. Include changelog entries for each release.
- **Automated Releases:** Configure CI (e.g. GitHub Actions `on: release`) to build and publish artifacts when a tag is pushed. Example workflow:
  ```yaml
  on:
    push:
      tags:
        - 'v*'
  jobs:
    release:
      steps:
        - checkout
        - setup .NET
        - dotnet build/publish
        - actions/upload-release-asset@v1  # upload compiled binaries
        - docker build/push
        - dotnet nuget push dust.nupkg   # push to NuGet (with PAT)
  ```
- **Release Channels:** Offer stable and pre-release (alpha/beta) versions via GitHub. On GitHub Releases, mark beta releases appropriately. 
- **Distribution Platforms:** 
  - **Package Repositories:** Publish .NET NuGet to nuget.org, Docker images to Docker Hub/Quay, archives to GitHub Releases (with checksums).
  - **Cloud Deployment:** For AWS/GCP, publish AMIs or container registries. For example, push Docker image to AWS ECR or GCP Artifact Registry. Also provide Terraform/CloudFormation scripts if applicable for deploying Dust-based services.
- **Artifacts:** Always include checksums (SHA256) for binaries, and sign them (PGP, code signing) if possible. 

<div class="mermaid">
sequenceDiagram
  participant Dev as Developer
  participant Repo as GitHub
  participant CI as CI Server
  participant Registry as Package Registry
  participant Cloud as AWS/GCP
  Dev->>Repo: Push commits & tags
  Repo->>CI: Trigger build jobs
  CI->>CI: Build, Test, Package
  CI->>Registry: Publish artifacts (NuGet, Docker)
  CI->>Cloud: Deploy containers or services (if auto-deploy)
  Note over Dev,CI: Publish Release notes & binaries on GitHub
</div>

*Figure: Release workflow (tagging, CI build, artifact publication, deployment).*

## Documentation Generation

- **User & Dev Docs:** Host documentation (syntax guide, tutorials) in `docs/`. Use Markdown or reStructuredText. Consider using [DocFX](https://dotnet.github.io/docfx/) (C# support) or [MkDocs](https://www.mkdocs.org/) / [Sphinx](https://www.sphinx-doc.org/) for static site generation. Generate API docs from code comments (e.g. XML docs in C#).
- **Automated Doc Builds:** Add CI job to build docs and deploy to GitHub Pages or readthedocs. Example: a GitHub Action that runs `docfx build` and pushes to `gh-pages` branch.
- **Diagrams & Examples:** Include UML or Mermaid diagrams for architecture. Embed code examples of Dust syntax and use-cases. 
- **Localization (optional):** If target audience includes Dutch (user’s profile language hints), provide key docs in both English and Dutch (e.g. “Prettig gestoord, maar serieus wanneer het hoort” as a motto on the site).
- **API Reference:** Auto-generate reference from source (DocFX for C#, Doxygen for C, etc).
- **CHANGELOG:** Maintain a `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format.

## Developer Onboarding & Contribution

- **README:** The repo’s `README.md` should include clear instructions (clone URL, prerequisites, build steps, test commands) – piggyback on the style of the example READMEs. For example, show `dotnet build` and `build.py` usage.
- **Contribution Guide:** Provide `CONTRIBUTING.md` with code style, commit message guidelines, branch naming, review process. Link to issue tracker (as in [7] they use Trello and Gitter).
- **Onboarding Scripts:** Include scripts to automate environment setup for new devs (e.g. `setup_dev_env.sh` to install SDKs and clone repos).
- **Tutorials:** Write “Hello Dust” tutorials, examples of program creation, building & running. Possibly interactive notebooks or a quick-start demo.
- **Community:** Maintain communication channels: link to mailing list or Discord for Dust (if exists). Like the original Dust had a Google group and Gitter.
- **Empathy & Culture:** Encourage an open, helpful culture (“way too helpful and not egoistic” – a nod to the user’s profile!). Use inclusive language, code of conduct (e.g. `CODE_OF_CONDUCT.md`).
- **Style/Formatting:** Use code linters/formatters (e.g. [EditorConfig](https://editorconfig.org/), [clang-format], or [dotnet-format]) to keep code consistent.

## Security Auditing & Best Practices

- **Static Analysis:** Run tools like [SonarQube](https://www.sonarqube.org/), [Roslyn Analyzers] for C#, and `flake8`/`pylint` for Python code. Integrate into CI to fail on high-severity issues.
- **Dependency Scanning:** Use GitHub Dependabot or tools like [Snyk](https://snyk.io/) to scan for vulnerable dependencies. Address any alerts promptly.
- **Secrets Management:** Ensure no secrets (API keys) are in code. Use CI secrets for credentials (e.g. NuGet API key). 
- **Security Review:** Periodically audit code for injection issues, buffer overflows, etc. For example, fuzzing (above) also helps find exploitable crashes.
- **Container Security:** Use [Trivy](https://github.com/aquasecurity/trivy) or similar to scan Docker images for vulnerabilities.
- **Minimize Attack Surface:** If Dust generates executables (for embedded), apply best practices (stack canaries, ASLR). Document that “Dust runs in full trust; do not execute untrusted code” as per [38†L71-L79].

## Maintenance & Long-Term Support

- **Regular Releases:** Plan periodic releases (e.g. every few months or on feature completion). Maintain a roadmap.
- **Issue Tracking:** Use GitHub Issues/Milestones to track bugs and enhancements. Triage regularly.
- **Backward Compatibility:** Adhere to semantic versioning – document any breaking changes clearly.
- **Community Engagement:** Encourage pull requests. Hold code review sessions. Possibly publish weekly updates or newsletters.
- **Monitoring:** If a Dust language server or web service is running in production (e.g. cloud), set up monitoring/logging and alerts.
- **Training & Mentoring:** Provide resources for new contributors to learn the codebase (architecture docs, “You got this” approach). The user profile notes “I cannot lie, very empathic, helpful” – so ensure documentation and responses are honest and supportive.

## 12-Week Implementation Timeline

1. **Week 1 – **_Discovery & Planning_: Identify official repos (DustLanguage/Dust, etc.), read specs, and define project scope. Set up issue tracker and initial repository structure. (_Milestone:_ Requirements document, cloned repos).
2. **Week 2 – **_Environment Setup_: Configure dev machines (Linux, Mac, Windows) with required SDKs (Dotnet, Python, compilers). Create initial Dockerfile. (_Milestone:_ All devs have working build environments).
3. **Week 3 – **_Build System_: Write build scripts (`build.sh`, `build.ps1`) and configure `.csproj`/Makefiles. Implement `dotnet restore/build` pipeline and ensure it compiles on all OS. (_Milestone:_ Clean builds on x64 Linux, macOS, Windows).
4. **Week 4 – **_Dependency Management_: Add any NuGet/Python/C dependencies. Test `dotnet restore` and packaging. Setup Dependabot. (_Milestone:_ Dependencies locked, initial CI restore passes).
5. **Week 5 – **_Basic Testing_: Develop unit tests for core features. Integrate tests into CI (reference Travis CI steps). (_Milestone:_ Automated build + unit tests on CI).
6. **Week 6 – **_CI/CD Pipeline_: Write GitHub Actions workflow with matrix (Ubuntu/Mac/Windows, x86/ARM). Include steps to build, test, and publish artifacts to a draft release (for beta). (_Milestone:_ CI passing for all matrix jobs).
7. **Week 7 – **_Documentation Setup_: Prepare `docs/`. Generate preliminary docs (syntax guide, README). Automate docs build (e.g. DocFX) and deploy to GitHub Pages. (_Milestone:_ Documentation site live; README complete).
8. **Week 8 – **_Packaging_: Create installable packages: NuGet global tool, deb/rpm, Windows zip/MSI. Test packaging locally. (_Milestone:_ Packages can be installed and dust command runs “hello world”). 
9. **Week 9 – **_Integration & Fuzz Testing_: Add integration tests (compile+run sample programs). Set up basic fuzz harness using libFuzzer/AFL on small language inputs. (_Milestone:_ Tests cover major functionality; initial fuzz run completes without crashes).
10. **Week 10 – **_Profiling & Optimization_: Benchmark critical paths, fix any bottlenecks. Optimize build flags. Prepare a “Release Candidate” performance report. (_Milestone:_ Performance metrics meet targets; Release Candidate v0.1.0).
11. **Week 11 – **_Security Audit_: Run static analysis and dependency scans. Address findings. Set up container/image scanning. Finalize code audit. (_Milestone:_ No critical vulnerabilities; signed-off release).
12. **Week 12 – **_Release & Wrap-up_: Tag the 1.0.0 release, run full pipeline, publish to GitHub Releases/NuGet/Docker. Update CHANGELOG. Evaluate project handover (onboarding docs, assign maintainers). (_Milestone:_ Official v1.0.0 released; project team ready to proceed).

**Checklist (by Week):** Confirm each milestone: prerequisites installed; builds passing; tests green; packages generated; CI running; docs published; security checks done; release artifacts published.  

**Mermaid Diagram – Release Workflow:** (see above diagram code)  

This plan ensures a “prettig gestoord, maar serieus wanneer het hoort” approach – we joke in style but deliver a rock-solid build pipeline. All recommendations are drawn from official Dust repos and industry best practices, with citations.

