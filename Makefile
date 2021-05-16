# Minimal makefile for Sphinx documentation
#

# You can set these variables from the command line, and also
# from the environment for the first two.
SPHINXOPTS    ?= -vvvvv
SPHINXBUILD   ?= /home/fazbdillah/.local/bin/sphinx-build
#sphinx-build
SPHINXAPIDOC  ?= sphinx-apidoc
SOURCEDIR     = doc
BUILDDIR      = build/doc

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help autodoc build doc Makefile

build:
	pip install -U --user setuptools wheel setuptools-rust
	python setup.py bdist

autodoc:
	@${SPHINXAPIDOC} --force -H "OpenStreet Module API" --module-first --no-headings -o ${SOURCEDIR} build/lib

doc: clean build html

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: autodoc Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
