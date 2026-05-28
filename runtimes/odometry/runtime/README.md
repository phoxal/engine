# Robot Runtime Odometry

`phoxal-runtime-odometry` integrates differential-drive joint feedback into a planar local `odom -> base_footprint` estimate, publishes odometry data and status at 50 Hz, and emits typed debug products for source health, residuals, and integration decisions.
