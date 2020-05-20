RLBOT_DIR ?= $(CURDIR)/../RLBot

help:
	@echo "usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "    ffi        Generate ffi bindings from RLBot headers using rust-bindgen"
	@echo "    fbs        Generate Flatbuffer bindings"

.PHONY: bindings fbs ffi signatures help

bindings: ffi fbs

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
		--no-copy ByteBuffer \
		--blacklist-type __uint8_t \
		$< \
		-- \
		-fdeclspec \
		-x c++ \
		-std=c++17 \
		-I "$(RLBOT_DIR)"/src/main/cpp/RLBotInterface/src/RLBotInterface \
		-I "$(RLBOT_DIR)"/src/main/cpp/RLBotInterface/src/RLBotMessages \
		-I "$(RLBOT_DIR)"/src/generated/cpp/flatbuffers

src/rlbot_generated.rs: fbs

fbs: $(RLBOT_DIR)/src/main/flatbuffers/rlbot.fbs
	flatc \
		--rust \
		-o src \
		$<
	cargo fix --allow-dirty
	cargo +nightly fmt

signatures: cpp/rlbot.hpp
	bindgen \
		--disable-name-namespacing \
		--no-layout-tests \
		--with-derive-default \
		--default-enum-style rust \
		--output function_signatures_generated.rs \
		--whitelist-function BallPrediction::GetBallPrediction \
		--whitelist-function BallPrediction::GetBallPredictionStruct \
		--whitelist-function GameFunctions::Free \
		--whitelist-function GameFunctions::SetGameState \
		--whitelist-function GameFunctions::StartMatch \
		--whitelist-function GameFunctions::StartMatchFlatbuffer \
		--whitelist-function GameFunctions::UpdateFieldInfoFlatbuffer \
		--whitelist-function GameFunctions::UpdateFieldInfo \
		--whitelist-function GameFunctions::UpdateLiveDataPacketFlatbuffer \
		--whitelist-function GameFunctions::UpdateLiveDataPacket \
		--whitelist-function GameFunctions::UpdateRigidBodyTickFlatbuffer \
		--whitelist-function GameFunctions::UpdateRigidBodyTick \
		--whitelist-function GameFunctions::SendQuickChat \
		--whitelist-function GameFunctions::SendChat \
		--whitelist-function GameFunctions::UpdatePlayerInput \
		--whitelist-function GameFunctions::UpdatePlayerInputFlatbuffer \
		--whitelist-function Interface::IsInitialized \
		--whitelist-function RenderFunctions::RenderGroup \
		--generate functions \
		$< \
		-- \
		-fdeclspec \
		-x c++ \
		-std=c++17 \
		-I "$(RLBOT_DIR)"/src/main/cpp/RLBotInterface/src/RLBotInterface \
		-I "$(RLBOT_DIR)"/src/main/cpp/RLBotInterface/src/RLBotMessages \
		-I "$(RLBOT_DIR)"/src/generated/cpp/flatbuffers
