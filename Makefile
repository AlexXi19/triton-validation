# Set the names of the Rust projects
CLIENT = client
SERVER = server
SIDECAR = sidecar

# Define the build targets for each project
.PHONY: build-client
build-client:
	cd $(CLIENT) && cargo build

.PHONY: build-server
build-server:
	cd $(SERVER) && cargo build

.PHONY: build-sidecar
build-sidecar:
	cd $(SIDECAR) && cargo build

# Define the clean targets for each project
.PHONY: clean-client
clean-client:
	cd $(CLIENT) && cargo clean

.PHONY: clean-server
clean-server:
	cd $(SERVER) && cargo clean

.PHONY: clean-sidecar
clean-sidecar:
	cd $(SIDECAR) && cargo clean

.PHONY: dev-client
dev-client:
	cd $(CLIENT) && cargo leptos watch

# Define the top-level build and clean targets
.PHONY: build clean

build: build-client build-server build-sidecar

clean: clean-client clean-server clean-sidecar
