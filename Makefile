#if we are building an example, just copy it to the projects/ folder
# and then treat it as a project
ifdef EXAMPLE
  #you can alternatively specify PROJECT= along with EXAMPLE=
  PROJECT?=exampleTempProject
  PROJECT_DIR?=$(abspath projects/$(PROJECT))
  $(shell mkdir -p $(abspath projects))
  $(shell rm -rf $(PROJECT_DIR))
  $(shell cp -r examples/$(EXAMPLE) $(PROJECT_DIR))
else
ifdef PROJECT
  PROJECT_DIR := $(abspath projects/$(PROJECT))
else
  PROJECT_DIR :=
endif
endif
