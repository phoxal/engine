FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        git \
        build-essential \
        cmake \
        pkg-config \
        libclang-dev \
        curl \
        ca-certificates \
        libeigen3-dev \
        libopencv-dev \
        libboost-all-dev \
        libssl-dev \
        libglew-dev \
        libpython3-dev \
        python3-dev \
        libgl1-mesa-dev \
        mesa-common-dev \
        libepoxy-dev \
        libxkbcommon-dev \
        libwayland-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /tmp
RUN git clone --depth 1 https://github.com/stevenlovegrove/Pangolin.git /tmp/Pangolin && \
    cmake -S /tmp/Pangolin -B /tmp/Pangolin/build \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_INSTALL_PREFIX=/usr/local \
        -DBUILD_EXAMPLES=OFF \
        -DBUILD_TOOLS=OFF \
        -DBUILD_TESTS=OFF && \
    cmake --build /tmp/Pangolin/build --parallel 4 && \
    cmake --install /tmp/Pangolin/build && \
    ln -sf /usr/local/lib/libpango_display.so /usr/local/lib/libpangolin.so && \
    rm -rf /tmp/Pangolin

# Pinned to the upstream V1.0 release (Dec 2021) — the latest ORB-SLAM3 line; the
# original UZ-SLAMLab repo is the canonical source (8.6k stars) and is frozen at
# this release. No actively-maintained fork supersedes it (community forks are
# build-doc/ROS wrappers or stale). Pinning keeps the build reproducible.
RUN git clone --depth 1 --branch v1.0-release https://github.com/UZ-SLAMLab/ORB_SLAM3.git /ORB_SLAM3

# GCC 11/OpenCV 4 builds need ORB-SLAM3 and bundled Sophus compiled as C++17.
RUN cd /ORB_SLAM3 && \
    sed -i 's/-std=c++11/-std=c++17/g; s/C++11/C++17/g; s/C11/C17/g' CMakeLists.txt && \
    sed -i 's/CMAKE_CXX_STANDARD 11/CMAKE_CXX_STANDARD 17/g; s/-std=c++11/-std=c++17/g' Thirdparty/Sophus/CMakeLists.txt

# C++17 rejects incrementing bool; ORB-SLAM3 uses mnFullBAIdx as a stop token, so assign it explicitly.
RUN cd /ORB_SLAM3 && \
    sed -i 's/mnFullBAIdx++;/mnFullBAIdx = true;/g' src/LoopClosing.cc

# Eigen 3.4 needs g2o's block permutation to use the same int storage index as its solver typedef.
RUN cd /ORB_SLAM3 && \
    sed -i 's/Eigen::PermutationMatrix<Eigen::Dynamic,Eigen::Dynamic> blockP;/Eigen::PermutationMatrix<Eigen::Dynamic,Eigen::Dynamic,int> blockP;/g' Thirdparty/g2o/g2o/solvers/linear_solver_eigen.h

# OpenCV 4 no longer exposes legacy image-codec/image-processing declarations through core headers.
RUN cd /ORB_SLAM3 && \
    set -eux; \
    add_include() { \
        file="$1"; \
        include="$2"; \
        if ! grep -qxF "$include" "$file"; then \
            line="$(grep -n '^#include' "$file" | head -n 1 | cut -d: -f1)"; \
            sed -i "${line}i ${include}" "$file"; \
        fi; \
    }; \
    for file in $(grep -rlE 'cv::(imread|imwrite)' src include Examples Examples_old); do \
        add_include "$file" '#include <opencv2/imgcodecs.hpp>'; \
    done; \
    for file in $(grep -rlE 'cv::(cvtColor|putText)' src include Examples Examples_old); do \
        add_include "$file" '#include <opencv2/imgproc.hpp>'; \
    done; \
    for file in $(grep -rl 'cv::imshow' src include Examples Examples_old); do \
        add_include "$file" '#include <opencv2/highgui.hpp>'; \
    done; \
    sed -i 's/CV_LOAD_IMAGE_UNCHANGED/cv::IMREAD_UNCHANGED/g; s/CV_GRAY2BGR/cv::COLOR_GRAY2BGR/g; s/CV_FONT_HERSHEY_DUPLEX/cv::FONT_HERSHEY_DUPLEX/g' src/LoopClosing.cc

# GCC 11/libstdc++ no longer exposes the old chrono monotonic_clock alias used by ORB-SLAM3 examples.
RUN cd /ORB_SLAM3 && \
    grep -rl 'std::chrono::monotonic_clock' Examples Examples_old | \
        xargs sed -i 's/std::chrono::monotonic_clock/std::chrono::steady_clock/g'

# Keep Docker builders from over-subscribing while still using upstream build.sh.
RUN cd /ORB_SLAM3 && \
    sed -i 's/make -j$/make -j4/g' build.sh && \
    ./build.sh && \
    echo '/usr/local/lib' > /etc/ld.so.conf.d/orb-slam3.conf && \
    echo '/ORB_SLAM3/lib' >> /etc/ld.so.conf.d/orb-slam3.conf && \
    echo '/ORB_SLAM3/Thirdparty/DBoW2/lib' >> /etc/ld.so.conf.d/orb-slam3.conf && \
    echo '/ORB_SLAM3/Thirdparty/g2o/lib' >> /etc/ld.so.conf.d/orb-slam3.conf && \
    ldconfig

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain stable --profile minimal

# Pangolin master installs component libraries; keep the legacy -lpangolin name
# expected by build.rs as a linker script over the installed components.
RUN rm -f /usr/local/lib/libpangolin.so && \
    printf '%s\n' 'GROUP ( libpango_display.so libpango_opengl.so libpango_windowing.so libpango_vars.so libpango_image.so libpango_core.so libepoxy.so libOpenGL.so libEGL.so )' \
        > /usr/local/lib/libpangolin.so

ENV ORB_SLAM3_DIR=/ORB_SLAM3
ENV PANGOLIN_DIR=/usr/local
# Pangolin's public headers expose modern OpenGL declarations behind HAVE_EPOXY;
# ORB-SLAM3's CMake receives that define, and the Rust wrapper build needs it too.
ENV CXXFLAGS="-I/usr/include/eigen3 -DHAVE_EPOXY" \
    BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include/eigen3 -DHAVE_EPOXY" \
    CPLUS_INCLUDE_PATH=/usr/include/eigen3

WORKDIR /workspace
COPY . /workspace
RUN cargo build --release -p phoxal-runtime-localize
RUN cargo test --release -p phoxal-runtime-localize \
        backend_runs_rgbd_inertial_ffi_lifecycle -- --nocapture --test-threads=1

ENV ORB_SLAM3_VOCABULARY=/ORB_SLAM3/Vocabulary/ORBvoc.txt

RUN mkdir -p /app && cp /workspace/target/release/phoxal-runtime-localize /app/phoxal-runtime-localize
CMD ["/app/phoxal-runtime-localize"]
