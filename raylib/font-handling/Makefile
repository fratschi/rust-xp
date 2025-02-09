FONT_DIR=font
FONT_URL=https://github.com/googlefonts/dm-mono/raw/main/exports/DMMono-Medium.ttf
FONT_FILE=$(FONT_DIR)/DMMono-Medium.ttf

.PHONY: all clean build run

all: $(FONT_FILE) build run

$(FONT_DIR):
	mkdir -p $(FONT_DIR)

$(FONT_FILE): | $(FONT_DIR)
	curl -L -o $(FONT_FILE) $(FONT_URL)

build: $(FONT_FILE)
	cargo build --release

run: build
	cargo run

clean:
	rm -rf $(FONT_DIR)
	cargo clean