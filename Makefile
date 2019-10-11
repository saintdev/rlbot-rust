RLBOT_DIR ?= $(CURDIR)/../RLBot

help:
	@echo "usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "    ffi        Generate ffi bindings from RLBot headers using rust-bindgen"

.PHONY: ffi help

src/ffi.rs: ffi

ffi: cpp/rlbot.hpp
	bindgen \
		--disable-name-namespacing \
		--no-layout-tests \
		--with-derive-default \
		--default-enum-style rust \
		--output src/ffi.rs \
		--raw-line '#![allow(non_camel_case_types, non_snake_case, missing_docs)]' \
		--whitelist-type ByteBuffer \
		--whitelist-type RLBotCoreStatus \
		--whitelist-type BallPredictionPacket \
		--whitelist-type MatchSettings \
		--whitelist-type FieldInfo \
		--whitelist-type LiveDataPacket \
		--whitelist-type RigidBodyTick \
		--whitelist-type QuickChatPreset \
		--whitelist-type CallbackFunction \
		$< \
		-- \
		-fdeclspec \
		-x c++ \
		-std=c++17 \
		-I "$(RLBOT_DIR)"/src/main/cpp/RLBotInterface/src/RLBotInterface \
		-I "$(RLBOT_DIR)"/src/main/cpp/RLBotInterface/src/RLBotMessages \
		-I "$(RLBOT_DIR)"/src/generated/cpp/flatbuffers
