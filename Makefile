## AT=   -- used instead of @ to silence the output. Defaults AT=@, use AT= for a very verbose output
AT?=@

build:
	$(shell mkdir -p ~/Bela/projects/bela-i2c)
	$(shell cp -r $(abspath libbela/render.cpp) ~/Bela/projects/bela-i2c)
	$(shell sh ~/Bela/scripts/build_projects.sh ~/Bela/projects/bela-i2c)
	#$(AT) echo ' ...done'
